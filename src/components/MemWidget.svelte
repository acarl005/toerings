<script lang="ts">
  export let memData: {
    ram: { usage: MemData; percentages: Array<number> }
    swap: { usage: MemData }
  }
  export let processList: Array<Process>

  import { toMetric, calcStrokeWidth } from "../lib/utils"
  import { foregroundColor } from "../lib/stores"
  import ArcStack from "./ArcStack.svelte"
  import ArcWidget from "./ArcWidget.svelte"
  import ProcessList from "./ProcessList.svelte"

  $: arcs = [
    {
      value: memData.ram.usage.mem_used_in_kib,
      max: memData.ram.usage.mem_total_in_kib,
      label: "RAM"
    },
    {
      value: memData.swap.usage.mem_used_in_kib,
      max: memData.swap.usage.mem_total_in_kib,
      label: "Swap"
    }
  ]

  $: attrs = [
    {
      key: "Available:",
      value: toMetric(memData.ram.usage.mem_total_in_kib * 1024)
    },
    { key: "Used:", value: toMetric(memData.ram.usage.mem_used_in_kib * 1024) }
  ]
  $: plotData = {
    x: memData.ram.percentages.map((_, i) => i),
    y: memData.ram.percentages,
    color: $foregroundColor
  }

  $: memSortedProcesses = [...processList]
    .sort((a, b) => a.mem_usage_bytes < b.mem_usage_bytes)
    .slice(0, 5)
</script>

<ArcWidget {attrs}>
  <ArcStack slot="arcStack" {arcs} size={120} strokeWidth={calcStrokeWidth(arcs.length)} />
  <ProcessList slot="content" title="Mem" plotDatas={[plotData]} processList={memSortedProcesses}>
    <span slot="processVal" let:process>{toMetric(process.mem_usage_bytes)}</span>
  </ProcessList>
</ArcWidget>
