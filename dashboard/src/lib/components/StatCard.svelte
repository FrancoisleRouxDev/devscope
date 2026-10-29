<!--
  =============================================================================
  StatCard.svelte — one headline number ("Total lines: 184,203")
  =============================================================================
  Purely presentational: receives already-computed values as props, formats
  nothing itself beyond calling format.ts helpers, fetches nothing.
  Used in a row at the top of <Overview />: Files · Lines · Languages · Scan time.
  =============================================================================
-->

<script lang="ts">
  // LOGIC SKETCH (commented out — not implemented yet)
  //
  // import { formatInt } from '../utils/format';
  //
  // let { label, value, hint }: { label: string; value: number; hint?: string } = $props();
</script>

<!--
  PLANNED MARKUP
  <div class="card">
    <span class="label">{label}</span>
    <span class="value num">{formatInt(value)}</span>
    {#if hint}<span class="hint">{hint}</span>{/if}
  </div>
-->
