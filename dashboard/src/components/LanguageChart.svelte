<!-- LanguageChart.svelte - language breakdown visualization (donut/bar). -->
<script>
  // languages: [{ name, files, code, comments, blanks }], already sorted by code lines.
  let { languages } = $props();

  // Total code lines across languages, used to turn each count into a percentage.
  let total = $derived(languages.reduce((sum, l) => sum + l.code, 0) || 1);
</script>

<section class="panel">
  <h2>Languages</h2>

  <!-- One stacked bar: each language's width is its share of code lines. -->
  <div class="bar">
    {#each languages as lang, i}
      <div
        style="width: {(lang.code / total) * 100}%; background: var(--series-{(i % 6) + 1})"
        title="{lang.name}: {lang.code}"
      ></div>
    {/each}
  </div>

  <!-- Legend: colour swatch, name and percentage per language. -->
  <ul>
    {#each languages as lang, i}
      <li>
        <span class="swatch" style="background: var(--series-{(i % 6) + 1})"></span>
        <span class="name">{lang.name}</span>
        <span class="pct">{((lang.code / total) * 100).toFixed(1)}%</span>
      </li>
    {/each}
  </ul>
</section>

<style>
  .panel { background: var(--surface); border: 1px solid var(--border); border-radius: var(--radius); padding: 20px; }
  h2 { font-size: 1rem; margin-bottom: 14px; }
  .bar { display: flex; height: 12px; border-radius: 6px; overflow: hidden; background: var(--surface-2); gap: 2px; }
  ul { list-style: none; padding: 0; margin-top: 16px; display: grid; grid-template-columns: repeat(auto-fill, minmax(180px, 1fr)); gap: 8px; }
  li { display: flex; align-items: center; gap: 8px; font-size: 0.9rem; }
  .swatch { width: 10px; height: 10px; border-radius: 3px; }
  .name { flex: 1; }
  .pct { color: var(--text-muted); font-variant-numeric: tabular-nums; }
</style>
