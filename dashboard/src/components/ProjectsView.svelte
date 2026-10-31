<!-- ProjectsView.svelte - the developer view: every code project in the folder,
     with git risk and reclaimable space, plus line stats and a secrets check
     for the project you click. -->
<script>
  import StatCard from './StatCard.svelte';
  import ProjectList from './ProjectList.svelte';
  import LanguageChart from './LanguageChart.svelte';
  import FileTable from './FileTable.svelte';
  import SecretsPanel from './SecretsPanel.svelte';
  import { formatBytes } from '../lib/format.js';

  // projects: ProjectOut[]; selected/result/findings describe the clicked project; onselect(project)
  let { projects, selected, result, findings, onselect } = $props();

  // Headline numbers for the whole folder, recomputed when `projects` changes.
  let totals = $derived({
    size: projects.reduce((s, p) => s + p.size_bytes, 0),
    junk: projects.reduce((s, p) => s + p.junk_bytes, 0),
    atRisk: projects.filter((p) => !p.git || p.git.uncommitted > 0 || p.git.ahead > 0 || !p.git.has_remote).length,
  });
</script>

<div class="stats">
  <StatCard label="Projects" value={projects.length} hint="Folders with .git, package.json, Cargo.toml…" />
  <StatCard label="Total size" text={formatBytes(totals.size)} />
  <StatCard label="Reclaimable" text={formatBytes(totals.junk)} hint="node_modules, target, venv…" tone="warn" />
  <StatCard label="Work at risk" value={totals.atRisk} hint="Uncommitted, unpushed or no git" tone="danger" />
</div>

<ProjectList {projects} {selected} {onselect} />

{#if selected && result}
  <h2 class="detail">{selected.name}</h2>
  <div class="stats">
    <StatCard label="Files" value={result.total_files} />
    <StatCard label="Lines" value={result.total_lines} />
    <StatCard label="Code" value={result.code} tone="ok" />
    <StatCard label="Comments" value={result.comments} />
  </div>
  <SecretsPanel {findings} />
  <LanguageChart languages={result.languages} />
  <FileTable files={result.largest_files} />
{/if}

<style>
  .stats { display: grid; grid-template-columns: repeat(auto-fit, minmax(190px, 1fr)); gap: var(--gap); }
  .detail { margin-top: 8px; font-size: 1.15rem; }
</style>
