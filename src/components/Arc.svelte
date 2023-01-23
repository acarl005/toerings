<script lang="ts">
  export let value = 0
  export let max = 100
  export let strokeColor = "rgba(255, 255, 255, 0.6)"
  export let strokeWidth = 8
  export let trackColor = "rgba(255, 255, 255, 0.2)"
  export let capColor = "white"
  export let size = 100
  export let label = null
  export let tooltip = null

  import { styleVars } from "../lib/actions"

  let showTooltip = false

  const margin = 5
  $: sizeHalf = size / 2
  $: zero = [sizeHalf, size - margin]
  $: x = zero[0]
  $: y = zero[1]
  $: fullPath = `M${zero[0]},${zero[1]} A${sizeHalf - margin},${sizeHalf - margin} 0 1,1 ${
    zero[1]
  },${zero[0]}`

  $: cssVars = {
    strokeColor,
    strokeWidth: `${strokeWidth}px`,
    trackColor,
    size: `${size}px`
  }

  let arcPath = ""
  $: if (value <= 0) {
    arcPath = ""
    x = zero[0]
    y = zero[1]
  } else {
    const frac = Math.min(value / max, 1)
    const angle = ((3 * Math.PI) / 2) * frac
    x = sizeHalf + Math.cos(angle + Math.PI / 2) * (sizeHalf - margin)
    y = sizeHalf + Math.sin(angle + Math.PI / 2) * (sizeHalf - margin)
    const majorArc = Number(angle > Math.PI)
    arcPath = `M${zero[0]},${zero[1]} A${sizeHalf - margin},${
      sizeHalf - margin
    } 0 ${majorArc},1 ${x},${y}`
  }
</script>

<div use:styleVars={cssVars} class="arc-wrap">
  <svg viewBox="0 0 {size} {size}" xmlns="http://www.w3.org/2000/svg">
    <path d={fullPath} />
    <path d={arcPath} />
    <circle cx={x} cy={y} r={strokeWidth / 2} fill={capColor} />
  </svg>
  {#if label}
    {#if showTooltip && tooltip}
      <div class="tooltip">
        {@html tooltip}
      </div>
    {/if}
    <div
      class="label"
      on:mouseenter={() => {
        showTooltip = true
      }}
      on:mouseleave={() => {
        showTooltip = false
      }}
    >
      {label}
    </div>
  {/if}
  {#if $$slots.default}
    <div>
      <div class="slot-parent">
        <slot {value} />
      </div>
    </div>
  {/if}
</div>

<style>
  svg {
    fill: transparent;
    position: absolute;
    stroke-linecap: round;
    left: 0;
  }

  path {
    stroke-width: var(--strokeWidth);
    stroke: var(--trackColor);
  }

  path:nth-child(2) {
    stroke: var(--strokeColor);
  }

  .arc-wrap {
    height: var(--size);
    width: var(--size);
    position: relative;
  }

  .label {
    position: absolute;
    bottom: -4px;
    left: calc(var(--size) / 2 + 7px);
    width: 45px;
    white-space: nowrap;
    overflow-x: hidden;
  }

  .slot-parent {
    position: absolute;
    left: 50%;
    top: 50%;
    transform: translate(-50%, -50%);
    height: fit-content;
    width: fit-content;
  }

  .tooltip {
    position: absolute;
    bottom: 20px;
    left: calc(var(--size) / 2 + 7px);
    background-color: white;
    color: black;
    padding: 5px 8px;
  }
</style>
