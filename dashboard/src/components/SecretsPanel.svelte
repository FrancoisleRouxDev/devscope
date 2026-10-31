<!-- SecretsPanel.svelte - possible leaked secrets in the selected project.
     The API never sends the secret itself, only where it was found. -->
<script>
  import Panel from './Panel.svelte';

  // findings: [{ path, line, kind }]
  let { findings } = $props();
</script>

<Panel title="Secrets check" subtitle="Key files and well-known token formats">
  {#if findings.length === 0}
    <p class="ok">No known secret patterns found.</p>
  {:else}
    <p class="warn">{findings.length} possible secret{findings.length === 1 ? '' : 's'} found. Review them and rotate any that are real.</p>
    <ul>
      {#each findings as f}
        <li>
          <span class="kind">{f.kind}</span>
          <span class="where" title={f.path}>{f.path}{f.line ? `:${f.line}` : ''}</span>
        </li>
      {/each}
    </ul>
  {/if}
</Panel>

<style>
  .ok { color: var(--ok); font-size: 0.9rem; }
  .warn { color: var(--danger); margin-bottom: 12px; font-size: 0.9rem; }
  ul { list-style: none; padding: 0; display: grid; gap: 6px; }
  li { display: flex; gap: 12px; font-size: 0.85rem; }
  .kind { color: var(--warn); min-width: 130px; font-weight: 600; }
  .where { font-family: var(--mono); font-size: 0.78rem; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; direction: rtl; text-align: left; }
</style>
