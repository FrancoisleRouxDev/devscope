<!-- ProjectList.svelte - every project found in the workspace, with size,
     reclaimable junk, last activity and git risk. Click a row to inspect it. -->
<script>
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

<section class="panel">
  <h2>Projects</h2>
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
</section>

<style>
  .panel { background: var(--surface); border: 1px solid var(--border); border-radius: var(--radius); padding: 20px; overflow-x: auto; }
  h2 { font-size: 1rem; margin-bottom: 14px; }
  table { width: 100%; border-collapse: collapse; font-size: 0.9rem; }
  th { text-align: left; color: var(--text-muted); font-weight: 600; padding: 8px 10px; }
  td { padding: 10px; border-top: 1px solid var(--border); }
  tbody tr { cursor: pointer; }
  tbody tr:hover, tbody tr.active { background: var(--surface-2); }
  .num { text-align: right; font-variant-numeric: tabular-nums; }
  .name { font-weight: 600; }
  .kinds { color: var(--text-muted); font-size: 0.78rem; }
  .badge { display: inline-block; margin: 2px 6px 2px 0; padding: 2px 8px; border-radius: 999px; font-size: 0.75rem; border: 1px solid currentColor; }
  .ok { color: var(--ok); }
  .warn { color: var(--series-4); }
  .danger { color: var(--danger); }
</style>
