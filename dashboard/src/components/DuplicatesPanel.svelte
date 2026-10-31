<!-- DuplicatesPanel.svelte - on-demand search for files with identical content.
     It is a separate step because it has to read file contents, which is slower. -->
<script>
  import Panel from './Panel.svelte';
  import { baseName, dirName, formatBytes } from '../lib/format.js';
  import { reveal } from '../lib/api.js';

  // groups: null = not searched yet, otherwise [{ size, paths, wasted }]
  let { groups, busy, onfind } = $props();

  let wasted = $derived((groups ?? []).reduce((s, g) => s + g.wasted, 0));
</script>

<Panel title="Duplicate files" subtitle="Files with exactly the same content, confirmed byte for byte.">
  {#snippet aside()}
    <button class="find" disabled={busy} onclick={onfind}>
      {#if busy}<span class="spinner"></span> Searching…{:else}{groups ? 'Search again' : 'Find duplicates'}{/if}
    </button>
  {/snippet}

  {#if groups === null}
    <p class="muted">Not searched yet. Run it to see which files are stored more than once.</p>
  {:else if groups.length === 0}
    <p class="ok">No duplicate files found.</p>
  {:else}
    <p class="summary"><strong>{formatBytes(wasted)}</strong> could be freed by keeping one copy of each ({groups.length} group{groups.length === 1 ? '' : 's'}).</p>
    <div class="groups">
      {#each groups as g (g.paths[0])}
        <div class="group">
          <div class="head">{g.paths.length} copies · {formatBytes(g.size)} each · <span class="waste">{formatBytes(g.wasted)} wasted</span></div>
          {#each g.paths as p}
            <div class="row">
              <span class="file" title={p}>{baseName(p)}</span>
              <span class="dir" title={dirName(p)}>{dirName(p)}</span>
              <button class="show" onclick={() => reveal(p)}>Show</button>
            </div>
          {/each}
        </div>
      {/each}
    </div>
  {/if}
</Panel>

<style>
  .muted { color: var(--text-muted); font-size: 0.88rem; }
  .ok { color: var(--ok); font-size: 0.88rem; }
  .summary { font-size: 0.9rem; margin-bottom: 12px; }
  .groups { display: grid; gap: 10px; max-height: 420px; overflow-y: auto; }
  .group { background: var(--surface-2); border-radius: var(--radius-sm); padding: 10px 12px; }
  .head { font-size: 0.8rem; color: var(--text-muted); margin-bottom: 6px; }
  .waste { color: var(--warn); font-weight: 600; }
  .row { display: flex; align-items: center; gap: 10px; padding: 3px 0; font-size: 0.85rem; }
  .file { font-weight: 600; flex: none; max-width: 40%; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
  .dir { flex: 1; min-width: 0; color: var(--text-muted); font-family: var(--mono); font-size: 0.72rem; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; direction: rtl; text-align: left; }
  button { border-radius: var(--radius-sm); font-size: 0.8rem; }
  .find { background: var(--accent-gradient); color: #fff; border: 0; padding: 7px 14px; font-weight: 600; display: inline-flex; align-items: center; gap: 8px; }
  .find:disabled { opacity: 0.7; cursor: progress; }
  .show { background: var(--bg); border: 1px solid var(--border); padding: 3px 9px; color: var(--text-muted); }
  .show:hover { color: var(--text); border-color: var(--accent); }
  .spinner { width: 12px; height: 12px; border: 2px solid rgba(255, 255, 255, 0.4); border-top-color: #fff; border-radius: 50%; animation: spin 0.8s linear infinite; }
</style>
