<!--
  =============================================================================
  Sidebar.svelte — left navigation column
  =============================================================================
  Shows the app name, an "Open project…" button (<ProjectPicker />), and the
  list of recently scanned projects (from GET /scans) so users can jump back.
  On macOS it leaves top padding for the traffic-light window buttons
  (titleBarStyle: 'hiddenInset' in electron/main.js) — check
  window.devscope?.platform === 'darwin'.
  =============================================================================
-->

<script lang="ts">
  // LOGIC SKETCH (commented out — not implemented yet)
  //
  // import ProjectPicker from './ProjectPicker.svelte';
  // import { scan } from '../stores/scan.svelte';
  //
  // const isMac = window.devscope?.platform === 'darwin';
</script>

<!--
  PLANNED MARKUP
  <aside class:mac={isMac}>
    <h1>devscope</h1>
    <ProjectPicker />
    <nav> {#each scan.recent as job (job.id)} <button onclick={() => scan.open(job)}>…</button> {/each} </nav>
  </aside>
-->
