<!-- Panel.svelte - the standard card used by every section: title bar + content.
     `children` is the body; the optional `aside` snippet puts something (a count,
     a button) on the right of the title. -->
<script>
  let { title, subtitle = '', children, aside } = $props();
</script>

<section class="panel">
  <header>
    <div>
      <h2>{title}</h2>
      {#if subtitle}<p>{subtitle}</p>{/if}
    </div>
    {@render aside?.()}
  </header>
  {@render children?.()}
</section>

<style>
  .panel {
    background: var(--surface);
    border: 1px solid var(--border-soft);
    border-radius: var(--radius);
    padding: 20px;
    box-shadow: var(--shadow);
    min-width: 0; /* lets long paths truncate inside grid columns */
    animation: rise 0.25s ease both;
  }
  header { display: flex; justify-content: space-between; align-items: flex-start; gap: 12px; margin-bottom: 16px; }
  h2 { font-size: 0.95rem; font-weight: 650; letter-spacing: 0.01em; }
  p { color: var(--text-muted); font-size: 0.8rem; margin-top: 3px; }
</style>
