<!-- ProjectList.svelte - every code project found in the folder, with size,
     reclaimable junk, last activity and git risk. Click a row to inspect it. -->
<script>
  import Panel from './Panel.svelte';
  import { formatBytes, timeAgo } from '../lib/format.js';

  // projects: ProjectOut[]; selected: the active project (or null); onselect(project)
  let { projects, selected, onselect } = $props();

  // Short git badges: what could be lost or forgotten in this project?
  function badges(p) {
    if (!p.git) return [{ text: 'no git', tone: 'warn' }];
    const out = [];
    if (p.git.uncommitted) out.push({ text: `${p.git.uncommitted} uncommitted`, tone: 'warn' });
    if (!p.git.has_remote) out.push({ text: 'no remote', tone: 'danger' });
    else if (p.git.ahead) out.push({ text: `${p.git.ahead} unpushed`, tone: 'warn' });
    if (!out.length) out.push({ text: 'clean', tone: 'ok' });
    return out;
  }
</script>

<Panel title="Projects" subtitle="Click a project for its details">
  <div class="scroll">
    <table>
      <thead>
        <tr><th>Project</th><th class="num">Size</th><th class="num">Reclaimable</th><th>Last activity</th><th>Git</th></tr>
      </thead>
      <tbody>
        {#each projects as p (p.path)}
          <tr class:active={selected?.path === p.path} onclick={() => onselect(p)}>
            <td>
              <div class="name">{p.name}</div>
              <div class="kinds">{p.kinds.join(' · ')}</div>
            </td>
            <td class="num">{formatBytes(p.size_bytes)}</td>
            <td class="num" title={p.junk_dirs.join(', ')}>{p.junk_bytes ? formatBytes(p.junk_bytes) : '–'}</td>
            <td>{timeAgo(p.last_modified)}</td>
            <td>
              {#each badges(p) as b}<span class="badge {b.tone}">{b.text}</span>{/each}
            </td>
          </tr>
        {/each}
      </tbody>
    </table>
  </div>
</Panel>

<style>
  .scroll { overflow-x: auto; }
  table { width: 100%; border-collapse: collapse; font-size: 0.88rem; }
  th { text-align: left; color: var(--text-muted); font-weight: 600; padding: 8px 10px; }
  td { padding: 11px 10px; border-top: 1px solid var(--border-soft); }
  tbody tr { cursor: pointer; transition: background 0.12s ease; }
  tbody tr:hover { background: var(--surface-2); }
  tbody tr.active { background: var(--surface-2); box-shadow: inset 3px 0 0 var(--accent); }
  .num { text-align: right; font-variant-numeric: tabular-nums; }
  .name { font-weight: 650; }
  .kinds { color: var(--text-muted); font-size: 0.76rem; }
  .badge { display: inline-block; margin: 2px 6px 2px 0; padding: 2px 9px; border-radius: 999px; font-size: 0.74rem; border: 1px solid currentColor; }
  .ok { color: var(--ok); }
  .warn { color: var(--warn); }
  .danger { color: var(--danger); }
</style>
