<!--
  =============================================================================
  views/Overview.svelte — the main dashboard for one scanned project
  =============================================================================
  Layout (CSS grid, collapses to one column below ~1000px):

    ┌ project name · path · [Rescan] ───────────────────────────────┐
    │ [Files]   [Lines of code]   [Languages]   [Scan time]         │  ← StatCard ×4
    ├───────────────────────────────┬───────────────────────────────┤
    │ LanguageBreakdown             │ LargestFiles                  │
    └───────────────────────────────┴───────────────────────────────┘

  Receives the report as a prop; derives the headline numbers with $derived.
  =============================================================================
-->

<script lang="ts">
  // LOGIC SKETCH (commented out — not implemented yet)
  //
  // import type { ScanReport } from '../lib/api/types';
  // import StatCard from '../lib/components/StatCard.svelte';
  // import LanguageBreakdown from '../lib/components/LanguageBreakdown.svelte';
  // import LargestFiles from '../lib/components/LargestFiles.svelte';
  //
  // let { report }: { report: ScanReport } = $props();
  // const codeLines = $derived(report.languages.reduce((s, l) => s + l.lines.code, 0));
</script>

<!--
  PLANNED MARKUP
  <header>…</header>
  <div class="stats"> <StatCard label="Files" value={report.totalFiles} /> … </div>
  <div class="grid"> <LanguageBreakdown languages={report.languages} /> <LargestFiles files={report.largestFiles} /> </div>
-->
