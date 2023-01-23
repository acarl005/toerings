<script lang="ts">
  export let summaryData: SummaryData
</script>

<header data-tauri-drag-region>
  <h1>System</h1>
  <div class="divider" />
</header>

<ul>
  <li><strong>Uptime:</strong> {summaryData.uptime}</li>
  <li><strong>OS Version:</strong> {summaryData.os_version}</li>
  <li>
    <strong>Kernel:</strong>
    {summaryData.kernel_name}
    {summaryData.kernel_version}
  </li>
</ul>

<style>
  header {
    display: flex;
    padding: 5px 10px 0 10px;
  }

  h1 {
    font-weight: 500;
    font-size: 13px;
    padding-right: 3px;
  }

  .divider {
    flex-grow: 1;
    border-top: 1px solid var(--foregroundColor);
    height: 0;
    align-self: center;
  }

  ul {
    margin-top: 0;
    margin-bottom: 20px;
    padding-left: 10px;
  }
</style>
