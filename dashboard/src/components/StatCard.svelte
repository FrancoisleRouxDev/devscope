<!-- StatCard.svelte - single headline metric (total files, total size, ...). -->
<script>
  // Pass `value` for a number (formatted with separators) or `text` for a ready-made string like "1.5 GB".
  // `tone` picks the accent colour of the top edge: accent | ok | warn | danger.
  let { label, value = 0, text = null, hint = '', tone = 'accent' } = $props();

  // `$derived` recomputes automatically when the inputs change.
  let shown = $derived(text ?? Number(value).toLocaleString());
</script>

<div class="card {tone}">
  <span class="label">{label}</span>
  <span class="value">{shown}</span>
  {#if hint}<span class="hint">{hint}</span>{/if}
</div>

<style>
  .card {
    position: relative;
    background: var(--surface);
    border: 1px solid var(--border-soft);
    border-radius: var(--radius);
    padding: 16px 18px 15px;
    display: flex;
    flex-direction: column;
    gap: 4px;
    overflow: hidden;
    box-shadow: var(--shadow);
    animation: rise 0.25s ease both;
  }
  /* coloured strip along the top edge */
  .card::before { content: ''; position: absolute; inset: 0 0 auto 0; height: 3px; background: var(--tone); }
  .accent { --tone: var(--accent-gradient); }
  .ok { --tone: var(--ok); }
  .warn { --tone: var(--warn); }
  .danger { --tone: var(--danger); }
  .label { color: var(--text-muted); font-size: 0.74rem; text-transform: uppercase; letter-spacing: 0.07em; font-weight: 600; }
  .value { font-size: 1.75rem; font-weight: 700; font-variant-numeric: tabular-nums; letter-spacing: -0.01em; }
  .hint { color: var(--text-muted); font-size: 0.78rem; }
</style>
