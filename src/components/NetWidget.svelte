<script>
  export let networkData
  export let localIp = null
  export let hostname

  import { calcStrokeWidth, toMetric } from "../lib/utils"
  import { uPlotAction } from "../lib/actions"
  import { titleColor, accentColor } from "../lib/stores"
  import ArcWidget from "./ArcWidget.svelte"
  import ArcStack from "./ArcStack.svelte"

  $: attrs = [
    { key: "Hostname:", value: hostname },
    { key: "Local IP:", value: localIp }
  ]

  $: arcs = [
    { label: "Down", value: networkData.rx.at(-1), max: 100 * 1024 ** 2 },
    { label: "Up", value: networkData.tx.at(-1), max: 100 * 1024 ** 2 }
  ]

  $: inPlotData = {
    x: networkData.rx.map((_, i) => i),
    y: networkData.rx,
    color: $accentColor
  }
  $: outPlotData = {
    x: networkData.tx.map((_, i) => i),
    y: networkData.tx,
    color: $titleColor
  }
</script>

<ArcWidget {attrs}>
  <ArcStack slot="arcStack" {arcs} size={120} strokeWidth={calcStrokeWidth(arcs.length)} />
  <div slot="content">
    <div class="flex">
      <h2>Net</h2>
      <div class="plot-container">
        <div use:uPlotAction={inPlotData} />
        <p class="caption">
          Down: {toMetric(networkData.rx.at(-1))}
        </p>
        <div use:uPlotAction={outPlotData} />
        <p class="caption">
          Up: {toMetric(networkData.tx.at(-1))}
        </p>
      </div>
    </div>
  </div>
</ArcWidget>

<style>
  .flex {
    display: flex;
  }

  h2 {
    margin: 0;
    font-size: 13px;
    font-weight: 500;
    padding-right: 5px;
    width: 48px;
    text-align: right;
    color: var(--titleColor);
  }

  .plot-container {
    position: relative;
  }

  .caption {
    text-align: right;
    margin-top: 5px;
  }
</style>
