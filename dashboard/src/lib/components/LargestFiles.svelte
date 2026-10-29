<!--
  =============================================================================
  LargestFiles.svelte — table of the biggest files by line count
  =============================================================================
  Renders report.largestFiles (already sorted and capped by the Rust scanner).
  Paths are relative and "/"-separated on every OS (normalised in Rust), and
  are rendered as TEXT — never with {@html} — because filenames are
  untrusted input.
  =============================================================================
-->

<script lang="ts">
  // LOGIC SKETCH (commented out — not implemented yet)
  //
  // import type { FileStats } from '../api/types';
  //
  // let { files }: { files: FileStats[] } = $props();
</script>

<!--
  PLANNED MARKUP
  <table>
    <thead><tr><th>File</th><th>Language</th><th>Lines</th><th>Size</th></tr></thead>
    <tbody> {#each files as f (f.path)} <tr><td class="mono">{f.path}</td>…</tr> {/each} </tbody>
  </table>
-->
