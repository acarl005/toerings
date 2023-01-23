<script lang="ts">
  export let title: string
  export let plotDatas: Array<{ x: Array<number>; y: Array<number> }>
  export let processList: Array<Process>

  import type { Colord } from "colord"
  import { uPlotAction, styleVars } from "../lib/actions"
  import { foregroundColor } from "../lib/stores"

  function attenuateLightness(color: Colord): Colord {
    const lightness = color.toHsl().l / 100 // 0 to 1
    const delta = lightness - 0.5
    return delta >= 0 ? color.darken(delta / 2) : color.lighten(delta / -2)
  }

  function halveSaturation(color: Colord): Colord {
    const hsl = color.toHsl()
    return color.desaturate(hsl.s / 2 / 100)
  }

  $: cssVars = {
    mutedForegroundColor: attenuateLightness(halveSaturation($foregroundColor)).toHslString()
  }
</script>

<div class="flex">
  <h2>{title}</h2>
  <div class="plot-container">
    {#each plotDatas as plotData, i}
      {#if i === 0}
        <div use:uPlotAction={plotData} />
      {:else}
        <div class="plot-wrap">
          <div use:uPlotAction={plotData} />
        </div>
      {/if}
    {/each}
  </div>
</div>
<ul class="processes" use:styleVars={cssVars}>
  {#each processList as process}
    <li>
      <div class="key">{process.name}</div>
      <div class="value">
        <slot name="processVal" {process} />
      </div>
    </li>
  {/each}
</ul>

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

  .plot-wrap {
    position: absolute;
    top: 0;
    left: 0;
  }

  .processes {
    margin: 0;
    padding-left: 40px;
  }

  .processes li {
    display: flex;
  }

  .processes li:first-child {
    color: var(--accentColor);
  }

  .processes li:nth-child(n + 3) {
    color: var(--mutedForegroundColor);
  }

  .key {
    flex-grow: 1;
    margin-right: 10px;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .value {
    text-align: right;
    white-space: nowrap;
  }
</style>
