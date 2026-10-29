<!--
  =============================================================================
  views/Welcome.svelte — empty state, shown before any project is scanned
  =============================================================================
  A centred call to action ("Open a project folder to see its stats") with a
  large <ProjectPicker />. Also the place to show "Starting engine…" while the
  Python sidecar boots, and a friendly error if it failed to start.
  =============================================================================
-->

<script lang="ts">
  // LOGIC SKETCH (commented out — not implemented yet)
  //
  // import ProjectPicker from '../lib/components/ProjectPicker.svelte';
  // import ScanProgress from '../lib/components/ScanProgress.svelte';
  // import { scan } from '../lib/stores/scan.svelte';
</script>

<!--
  PLANNED MARKUP
  {#if scan.status === 'connecting'} Starting engine…
  {:else if scan.status === 'scanning'} <ScanProgress path={scan.job.request.path} />
  {:else} <h2>Open a project folder to see its stats</h2> <ProjectPicker /> {/if}
-->
