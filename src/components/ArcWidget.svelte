<script lang="ts">
  export let attrs: Array<{ key: string; value: string }>
</script>

<div class="container">
  <div class="arc-wrap">
    <slot name="arcStack" />
  </div>
  <ul>
    {#each attrs as pair}
      <li>
        <div class="key">{pair.key}</div>
        <div class="value">{pair.value}</div>
      </li>
    {/each}
  </ul>
  <div class="flowing-content">
    <slot name="content" />
  </div>
</div>

<style>
  .container {
    position: relative;
    display: flex;
    flex-direction: column;
    justify-content: center;
    margin-bottom: 30px;
  }

  .arc-wrap {
    position: absolute;
    top: 0;
    left: 0;
    z-index: 2;
  }

  ul {
    position: absolute;
    top: 0;
    right: 12px;
    text-align: right;
    min-width: 150px;
  }

  li {
    display: flex;
    justify-content: space-between;
  }

  .key {
    font-weight: bold;
    margin-right: 6px;
  }

  .flowing-content {
    margin-top: 65px;
    margin-left: 75px;
  }
</style>
