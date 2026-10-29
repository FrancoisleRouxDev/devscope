<!--
  =============================================================================
  LanguageBreakdown.svelte — per-language chart + table
  =============================================================================
  Shows each language's share of code lines as a horizontal stacked bar (plain
  SVG/CSS — no chart library needed for this) plus a table:
      Language | Files | Code | Comments | Blanks | %
  Colours come from --chart-1…--chart-6 in app.css; languages beyond the 6th
  are grouped into "Other" so the palette stays legible.
  Accessibility: every bar segment has a text label/tooltip; colour is never
  the only way to tell languages apart.
  =============================================================================
-->

<script lang="ts">
  // LOGIC SKETCH (commented out — not implemented yet)
  //
  // import type { LanguageStats } from '../api/types';
  //
  // let { languages }: { languages: LanguageStats[] } = $props();
  //
  // const totalCode = $derived(languages.reduce((sum, l) => sum + l.lines.code, 0));
  // const top = $derived(groupTail(languages, 6));   // top 5 + "Other"
</script>

<!--
  PLANNED MARKUP
  <section>
    <div class="bar"> {#each top as l, i} <span style:flex-grow={l.lines.code} style:background="var(--chart-{i + 1})" title=…></span> {/each} </div>
    <table> … </table>
  </section>
-->
