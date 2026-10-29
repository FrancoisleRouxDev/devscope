<!--
  =============================================================================
  src/App.svelte — root component / app shell
  =============================================================================
  Layout:
    ┌──────────┬─────────────────────────────────────────────┐
    │ Sidebar  │  <Welcome />   when no project is selected  │
    │ (recent  │  <Overview />  once a scan has a report     │
    │ projects)│                                             │
    └──────────┴─────────────────────────────────────────────┘

  Responsibilities:
    * On mount, wait for the API connection (window.devscope.getApiConnection)
      and show a "Starting engine…" state until it's ready.
    * Choose which view to render from the scan store's state.
    * Nothing else — no fetching or formatting here.
  =============================================================================
-->

<script lang="ts">
  // ===========================================================================
  // LOGIC SKETCH (commented out — not implemented yet)
  // ===========================================================================
  //
  // import Sidebar from './lib/components/Sidebar.svelte';
  // import Welcome from './views/Welcome.svelte';
  // import Overview from './views/Overview.svelte';
  // import { scan } from './lib/stores/scan.svelte';
  //
  // $effect(() => { scan.connect(); });     // resolve API base URL + token once
</script>

<!--
  PLANNED MARKUP
  <div class="shell">
    <Sidebar />
    <main>
      {#if scan.report} <Overview report={scan.report} /> {:else} <Welcome /> {/if}
    </main>
  </div>
-->
