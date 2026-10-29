<!--
  =============================================================================
  ProjectPicker.svelte — "Open project…" button (+ drag-and-drop target)
  =============================================================================
  Calls window.devscope.pickFolder() (native OS dialog via Electron main) and
  hands the chosen absolute path to scan.scanPath(). The renderer never reads
  the filesystem itself — it only passes the path string to the API.

  Later: accept a folder dropped onto the window. In Electron, use
  webUtils.getPathForFile(file) exposed via preload (File.path was removed).
  =============================================================================
-->

<script lang="ts">
  // LOGIC SKETCH (commented out — not implemented yet)
  //
  // import { scan } from '../stores/scan.svelte';
  //
  // let { compact = false }: { compact?: boolean } = $props();
</script>

<!--
  PLANNED MARKUP
  <button onclick={() => scan.pickAndScan()} disabled={scan.status === 'scanning'}>
    Open project…
  </button>
-->
