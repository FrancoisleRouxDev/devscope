<!-- BarList.svelte - rows with a label, a proportional bar and a value.
     Used for file categories, file ages and the biggest subfolders. -->
<script>
  // items: [{ label, sub?, value, display, color? }]
  // `value` sizes the bar (relative to the largest); `display` is the text shown on the right.
  let { items } = $props();

  let max = $derived(Math.max(...items.map((i) => i.value), 1));
</script>

<ul>
  {#each items as item (item.label)}
    <li>
      <div class="top">
        <span class="label">{item.label}</span>
        {#if item.sub}<span class="sub">{item.sub}</span>{/if}
        <span class="display">{item.display}</span>
      </div>
      <div class="track">
        <div class="fill" style="width: {Math.max((item.value / max) * 100, item.value ? 2 : 0)}%; background: {item.color ?? 'var(--accent-gradient)'}"></div>
      </div>
    </li>
  {/each}
</ul>

<style>
  ul { list-style: none; padding: 0; display: grid; gap: 12px; }
  .top { display: flex; align-items: baseline; gap: 8px; margin-bottom: 5px; font-size: 0.88rem; }
  .label { font-weight: 600; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
  .sub { color: var(--text-muted); font-size: 0.78rem; white-space: nowrap; }
  .display { margin-left: auto; font-variant-numeric: tabular-nums; color: var(--text-muted); white-space: nowrap; }
  .track { height: 7px; background: var(--surface-2); border-radius: 999px; overflow: hidden; }
  .fill { height: 100%; border-radius: 999px; transition: width 0.4s ease; }
</style>
