<!-- FileList.svelte - a list of files with size and age; the Show button reveals
     the file in the system file manager. -->
<script>
  import { baseName, dirName, formatBytes, timeAgo, categoryColor } from '../lib/format.js';
  import { reveal } from '../lib/api.js';

  // files: [{ path, size, modified, category, reason? }]
  let { files, empty = 'Nothing to show.' } = $props();

  const reasons = { installer: 'Installer', copy: 'Repeat download' };
</script>

{#if files.length === 0}
  <p class="empty">{empty}</p>
{:else}
  <ul>
    {#each files as f (f.path)}
      <li>
        <span class="dot" style="background: {categoryColor(f.category)}" title={f.category}></span>
        <div class="name">
          <span class="file" title={baseName(f.path)}>{baseName(f.path)}</span>
          <span class="dir" title={dirName(f.path)}>{dirName(f.path)}</span>
        </div>
        {#if f.reason}<span class="tag">{reasons[f.reason] ?? f.reason}</span>{/if}
        <span class="age">{timeAgo(f.modified)}</span>
        <span class="size">{formatBytes(f.size)}</span>
        <button onclick={() => reveal(f.path)} title="Show in file manager">Show</button>
      </li>
    {/each}
  </ul>
{/if}

<style>
  ul { list-style: none; padding: 0; }
  li { display: flex; align-items: center; gap: 12px; padding: 9px 4px; border-top: 1px solid var(--border-soft); font-size: 0.88rem; }
  li:first-child { border-top: 0; }
  .dot { width: 9px; height: 9px; border-radius: 50%; flex: none; }
  .name { display: flex; flex-direction: column; min-width: 0; flex: 1; }
  .file { font-weight: 600; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
  .dir { color: var(--text-muted); font-family: var(--mono); font-size: 0.72rem; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; direction: rtl; text-align: left; }
  .tag { font-size: 0.72rem; color: var(--warn); border: 1px solid currentColor; border-radius: 999px; padding: 1px 8px; white-space: nowrap; }
  .age { color: var(--text-muted); font-size: 0.8rem; white-space: nowrap; }
  .size { font-variant-numeric: tabular-nums; min-width: 64px; text-align: right; font-weight: 600; }
  button { background: var(--surface-2); border: 1px solid var(--border); border-radius: var(--radius-sm); padding: 4px 10px; font-size: 0.78rem; color: var(--text-muted); }
  button:hover { color: var(--text); border-color: var(--accent); }
  .empty { color: var(--text-muted); font-size: 0.88rem; }
</style>
