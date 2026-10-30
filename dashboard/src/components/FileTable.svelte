<!-- FileTable.svelte - sortable table of largest files / per-language details. -->
<script>
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
</script>

<section class="panel">
  <h2>Largest files</h2>
  <table>
    <thead>
      <tr>
        <th><button onclick={() => sortBy('path')}>File</button></th>
        <th><button onclick={() => sortBy('language')}>Language</button></th>
        <th class="num"><button onclick={() => sortBy('code')}>Code</button></th>
        <th class="num"><button onclick={() => sortBy('comments')}>Comments</button></th>
        <th class="num"><button onclick={() => sortBy('blanks')}>Blank</button></th>
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
</section>

<style>
  .panel { background: var(--surface); border: 1px solid var(--border); border-radius: var(--radius); padding: 20px; overflow-x: auto; }
  h2 { font-size: 1rem; margin-bottom: 14px; }
  table { width: 100%; border-collapse: collapse; font-size: 0.9rem; }
  th, td { text-align: left; padding: 8px 10px; border-bottom: 1px solid var(--border); }
  th button { background: none; border: 0; color: var(--text-muted); font-weight: 600; }
  .num { text-align: right; font-variant-numeric: tabular-nums; }
  .path { font-family: var(--mono); max-width: 360px; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; direction: rtl; text-align: left; }
</style>
