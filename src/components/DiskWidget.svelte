<script lang="ts">
  export let diskData: Array<DiskData>
  export let ioData: Array<{ read: number; write: number }>
  export let processList: Array<Process>

  import { basename } from "path"
  import { uniqBy } from "lodash-es"

  import { toMetric, calcStrokeWidth } from "../lib/utils"
  import { foregroundColor } from "../lib/stores"
  import ArcStack from "./ArcStack.svelte"
  import ArcWidget from "./ArcWidget.svelte"
  import ProcessList from "./ProcessList.svelte"

  function formatPath(filepath: string): string {
    if (filepath === "/") {
      return "/"
    }
    return basename(filepath)
  }

  $: pathSortedDisks = uniqBy(
    [...diskData].sort((a, b) => a.mount_point > b.mount_point),
    disk => `${disk.free_space},${disk.used_space},${disk.total_space}`
  )

  $: ioSortedProcesses = [...processList]
    .sort(
      (a, b) =>
        a.write_bytes_per_sec + a.read_bytes_per_sec < b.write_bytes_per_sec + b.read_bytes_per_sec
    )
    .slice(0, 5)

  $: arcs = pathSortedDisks.slice(0, 4).map(disk => ({
    label: formatPath(disk.mount_point),
    value: disk.used_space,
    max: disk.total_space,
    tooltip: `${disk.name}<br/>${disk.mount_point}<br/>${(
      (disk.used_space / disk.total_space) *
      100
    ).toFixed(1)}% used`
  }))

  $: attrs = [
    { key: "Read:", value: toMetric(ioData.at(-1).read) },
    { key: "Write:", value: toMetric(ioData.at(-1).write) }
  ]

  $: plotDatas = [
    {
      x: ioData.map((_, i) => i),
      y: ioData.map(point => point.read),
      color: $foregroundColor
    },
    {
      x: ioData.map((_, i) => i),
      y: ioData.map(point => point.write),
      color: $foregroundColor
    }
  ]
</script>

<ArcWidget {attrs}>
  <ArcStack slot="arcStack" {arcs} size={120} strokeWidth={calcStrokeWidth(arcs.length)} />
  <ProcessList slot="content" title="Disk" {plotDatas} processList={ioSortedProcesses}>
    <span slot="processVal" let:process>
      r:{toMetric(process.read_bytes_per_sec, 0)}, w:{toMetric(process.write_bytes_per_sec, 0)}
    </span>
  </ProcessList>
</ArcWidget>
