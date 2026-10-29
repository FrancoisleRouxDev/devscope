<!--
  =============================================================================
  ScanProgress.svelte — "Scanning ~/code/big-repo…" indicator
  =============================================================================
  Shown while scan.status === 'scanning'. Indeterminate for v1 (the API only
  reports queued/running/done). When the scanner gains progress callbacks,
  this shows "N files scanned" from the job status instead.
  Respects prefers-reduced-motion (no spinning animation when set).
  =============================================================================
-->

<script lang="ts">
  // LOGIC SKETCH (commented out — not implemented yet)
  //
  // let { path }: { path: string } = $props();
</script>

<!--
  PLANNED MARKUP
  <div role="status" aria-live="polite"> <span class="spinner"></span> Scanning {path}… </div>
-->
