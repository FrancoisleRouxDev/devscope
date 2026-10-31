<!-- FolderView.svelte - the general overview of any folder: what takes the space,
     how old it is, and what looks like clutter. -->
<script>
  import Panel from './Panel.svelte';
  import StatCard from './StatCard.svelte';
  import BarList from './BarList.svelte';
  import FileList from './FileList.svelte';
  import DuplicatesPanel from './DuplicatesPanel.svelte';
  import { formatBytes, categoryColor } from '../lib/format.js';

  // report: FolderResponse; dupes: null | groups; dupesBusy: bool; onfinddupes(): start the search
  let { report, dupes, dupesBusy, onfinddupes } = $props();

  // Rows for each BarList, rebuilt automatically when `report` changes.
  let categories = $derived(
    report.categories.map((c) => ({
      label: c.name,
      sub: `${c.files.toLocaleString()} file${c.files === 1 ? '' : 's'}`,
      value: c.bytes,
      display: formatBytes(c.bytes),
      color: categoryColor(c.name),
    })),
  );
  let ages = $derived(
    report.age_buckets.map((a) => ({
      label: a.label,
      sub: `${a.files.toLocaleString()} file${a.files === 1 ? '' : 's'}`,
      value: a.bytes,
      display: formatBytes(a.bytes),
    })),
  );
  let folders = $derived(
    report.largest_dirs.filter((d) => d.bytes > 0).slice(0, 8).map((d) => ({
      label: d.name,
      sub: `${d.files.toLocaleString()} files`,
      value: d.bytes,
      display: formatBytes(d.bytes),
    })),
  );

  // Space held by files untouched for a year or more (the two oldest buckets).
  let oldBytes = $derived(report.age_buckets.slice(3).reduce((s, a) => s + a.bytes, 0));
  let clutterBytes = $derived(report.clutter.reduce((s, f) => s + f.size, 0));
</script>

<div class="stats">
  <StatCard label="Total size" text={formatBytes(report.total_bytes)} hint="{report.total_files.toLocaleString()} files" />
  <StatCard label="Untouched 1 yr+" text={formatBytes(oldBytes)} hint="Old files you may not need" tone="warn" />
  <StatCard label="Likely clutter" text={formatBytes(clutterBytes)} hint="{report.clutter.length} installers and repeat downloads" tone="danger" />
  <StatCard label="Scanned in" text="{report.duration_ms} ms" hint="Read-only, nothing changed" tone="ok" />
</div>

<div class="grid">
  <Panel title="What's taking space" subtitle="By file type">
    {#if categories.length}<BarList items={categories} />{:else}<p class="muted">This folder is empty.</p>{/if}
  </Panel>
  <Panel title="How old is it" subtitle="By last change">
    <BarList items={ages} />
  </Panel>
</div>

{#if folders.length}
  <Panel title="Biggest subfolders">
    <BarList items={folders} />
  </Panel>
{/if}

<Panel title="Largest files" subtitle="Top {report.largest_files.length}">
  <FileList files={report.largest_files} empty="No files in this folder." />
</Panel>

<Panel title="Likely clutter" subtitle="Installers you have probably already used, and repeat downloads like “file (1).pdf”">
  <FileList files={report.clutter} empty="No obvious clutter found. Nice and tidy." />
</Panel>

<DuplicatesPanel groups={dupes} busy={dupesBusy} onfind={onfinddupes} />

<style>
  .stats { display: grid; grid-template-columns: repeat(auto-fit, minmax(190px, 1fr)); gap: var(--gap); }
  .grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(340px, 1fr)); gap: var(--gap); }
  .muted { color: var(--text-muted); font-size: 0.88rem; }
</style>
