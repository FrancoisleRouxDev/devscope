<!-- FileTable.svelte - sortable table of the largest source files in a project. -->
<script>
  import Panel from './Panel.svelte';

  // files: [{ path, language, code, comments, blanks }]
  let { files } = $props();

  let sortKey = $state('code');   // `$state` = reactive local variable
  let descending = $state(true);

  // Re-sorted automatically whenever `files`, `sortKey` or `descending` change.
  let rows = $derived(
    [...files].sort((a, b) => {
      const x = a[sortKey], y = b[sortKey];
      const order = typeof x === 'string' ? x.localeCompare(y) : x - y;
      return descending ? -order : order;
    }),
  );

  // Clicking the active column flips direction; clicking a new one sorts by it.
  function sortBy(key) {
    if (sortKey === key) descending = !descending;
    else { sortKey = key; descending = true; }
  }

  const arrow = (key) => (sortKey === key ? (descending ? ' ↓' : ' ↑') : '');
</script>

<Panel title="Largest source files" subtitle="Click a column to sort">
  <div class="scroll">
    <table>
      <thead>
        <tr>
          <th><button onclick={() => sortBy('path')}>File{arrow('path')}</button></th>
          <th><button onclick={() => sortBy('language')}>Language{arrow('language')}</button></th>
          <th class="num"><button onclick={() => sortBy('code')}>Code{arrow('code')}</button></th>
          <th class="num"><button onclick={() => sortBy('comments')}>Comments{arrow('comments')}</button></th>
          <th class="num"><button onclick={() => sortBy('blanks')}>Blank{arrow('blanks')}</button></th>
        </tr>
      </thead>
      <tbody>
        {#each rows as f (f.path)}
          <tr>
            <td class="path" title={f.path}>{f.path}</td>
            <td>{f.language}</td>
            <td class="num">{f.code.toLocaleString()}</td>
            <td class="num">{f.comments.toLocaleString()}</td>
            <td class="num">{f.blanks.toLocaleString()}</td>
          </tr>
        {/each}
      </tbody>
    </table>
  </div>
</Panel>

<style>
  .scroll { overflow-x: auto; }
  table { width: 100%; border-collapse: collapse; font-size: 0.88rem; }
  th, td { text-align: left; padding: 9px 10px; border-top: 1px solid var(--border-soft); }
  thead th { border-top: 0; }
  th button { background: none; border: 0; color: var(--text-muted); font-weight: 600; padding: 0; }
  th button:hover { color: var(--text); }
  .num { text-align: right; font-variant-numeric: tabular-nums; }
  .path { font-family: var(--mono); font-size: 0.78rem; max-width: 360px; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; direction: rtl; text-align: left; }
</style>
