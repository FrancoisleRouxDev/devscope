<!-- StatCard.svelte - single headline metric (total files, total lines, ...). -->
<script>
  let { label, value } = $props();

  // `$derived` recomputes automatically when `value` changes: 12345 -> "12,345".
  let formatted = $derived(Number(value).toLocaleString());
</script>

<div class="card">
  <span class="label">{label}</span>
  <span class="value">{formatted}</span>
</div>

<style>
  .card {
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: var(--radius);
    padding: 16px 20px;
    display: flex;
    flex-direction: column;
    gap: 6px;
  }
  .label { color: var(--text-muted); font-size: 0.8rem; text-transform: uppercase; letter-spacing: 0.05em; }
  .value { font-size: 1.9rem; font-weight: 700; font-variant-numeric: tabular-nums; }
</style>
