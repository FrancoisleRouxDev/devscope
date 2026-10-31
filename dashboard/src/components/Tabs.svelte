<!-- Tabs.svelte - switches between the folder overview and the projects view. -->
<script>
  // items: [{ id, label, count? }]; active: id of the selected tab; onchange(id)
  let { items, active, onchange } = $props();
</script>

<div class="tabs" role="tablist">
  {#each items as t (t.id)}
    <button role="tab" aria-selected={active === t.id} class:active={active === t.id} onclick={() => onchange(t.id)}>
      {t.label}
      {#if t.count != null}<span class="count">{t.count}</span>{/if}
    </button>
  {/each}
</div>

<style>
  .tabs { display: flex; gap: 4px; border-bottom: 1px solid var(--border-soft); }
  button {
    background: none;
    border: 0;
    padding: 10px 14px;
    color: var(--text-muted);
    font-weight: 600;
    border-bottom: 2px solid transparent;
    margin-bottom: -1px;
  }
  button:hover { color: var(--text); }
  button.active { color: var(--text); border-bottom-color: var(--accent); }
  .count { margin-left: 6px; padding: 1px 7px; border-radius: 999px; background: var(--surface-2); font-size: 0.75rem; }
</style>
