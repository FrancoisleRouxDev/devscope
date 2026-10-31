<!-- SecretsPanel.svelte - possible leaked secrets in the selected project.
     The API never sends the secret itself, only where it was found. -->
<script>
  // findings: [{ path, line, kind }]
  let { findings } = $props();
</script>

<section class="panel">
  <h2>Secrets check</h2>
  {#if findings.length === 0}
    <p class="ok">No known secret patterns found.</p>
  {:else}
    <p class="warn">{findings.length} possible secret{findings.length === 1 ? '' : 's'} found. Review and rotate any real ones.</p>
    <ul>
      {#each findings as f}
        <li>
          <span class="kind">{f.kind}</span>
          <span class="where" title={f.path}>{f.path}{f.line ? `:${f.line}` : ''}</span>
        </li>
      {/each}
    </ul>
  {/if}
</section>

<style>
  .panel { background: var(--surface); border: 1px solid var(--border); border-radius: var(--radius); padding: 20px; }
  h2 { font-size: 1rem; margin-bottom: 10px; }
  .ok { color: var(--ok); }
  .warn { color: var(--danger); margin-bottom: 10px; }
  ul { list-style: none; padding: 0; display: grid; gap: 6px; }
  li { display: flex; gap: 12px; font-size: 0.85rem; }
  .kind { color: var(--series-4); min-width: 130px; }
  .where { font-family: var(--mono); overflow: hidden; text-overflow: ellipsis; white-space: nowrap; direction: rtl; text-align: left; }
</style>
