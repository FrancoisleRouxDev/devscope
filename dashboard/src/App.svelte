<!-- App.svelte - root component. Lays out the dashboard shell:
     header + folder picker, workspace overview, then details for the
     project the user clicked. -->
<script>
  import { onMount } from 'svelte';
  import { connect, pickFolder, workspace, scan, secrets } from './lib/api.js';
  import { formatBytes } from './lib/format.js';
  import { apiOnline, root, projects, selected, result, findings, loading, error } from './lib/stores.js';
  import ProjectPicker from './components/ProjectPicker.svelte';
  import StatCard from './components/StatCard.svelte';
  import ProjectList from './components/ProjectList.svelte';
  import LanguageChart from './components/LanguageChart.svelte';
  import FileTable from './components/FileTable.svelte';
  import SecretsPanel from './components/SecretsPanel.svelte';

  // Runs once when the window opens: find out whether the API is up.
  onMount(async () => {
    $apiOnline = await connect();
  });

  // Run an API task with shared loading/error handling.
  async function run(task) {
    $error = null;
    $loading = true;
    try {
      await task();
    } catch (e) {
      $error = e.message;
    } finally {
      $loading = false;
    }
  }

  // "Choose workspace": pick a folder, then find every project inside it.
  async function chooseWorkspace() {
    const path = await pickFolder();
    if (!path) return; // dialog cancelled
    $root = path;
    $selected = null;
    $result = null;
    await run(async () => {
      $projects = await workspace(path);
    });
  }

  // Row click: load line stats and the secrets check for that project in parallel.
  async function inspect(project) {
    $selected = project;
    await run(async () => {
      [$result, $findings] = await Promise.all([scan(project.path), secrets(project.path)]);
    });
  }

  // Headline numbers for the whole workspace, recomputed when `$projects` changes.
  let totals = $derived({
    count: $projects.length,
    size: $projects.reduce((s, p) => s + p.size_bytes, 0),
    junk: $projects.reduce((s, p) => s + p.junk_bytes, 0),
    atRisk: $projects.filter((p) => p.git && (p.git.uncommitted > 0 || p.git.ahead > 0)).length,
  });
</script>

<main>
  <header>
    <div>
      <h1>devscope</h1>
      <p class="path">{$root ?? 'No workspace selected'}</p>
    </div>
    <div class="right">
      <span class="status" class:online={$apiOnline}>{$apiOnline ? 'API online' : 'API offline'}</span>
      <ProjectPicker onpick={chooseWorkspace} disabled={$loading || !$apiOnline} label="Choose workspace…" />
    </div>
  </header>

  {#if $error}
    <p class="error">{$error}</p>
  {/if}

  {#if $projects.length}
    <div class="stats">
      <StatCard label="Projects" value={totals.count} />
      <div class="text-stat"><span>Total size</span><strong>{formatBytes(totals.size)}</strong></div>
      <div class="text-stat"><span>Reclaimable</span><strong>{formatBytes(totals.junk)}</strong></div>
      <StatCard label="With unsaved work" value={totals.atRisk} />
    </div>

    <ProjectList projects={$projects} selected={$selected} onselect={inspect} />

    {#if $selected && $result}
      <h2 class="detail-title">{$selected.name}</h2>
      <div class="stats">
        <StatCard label="Files" value={$result.total_files} />
        <StatCard label="Lines" value={$result.total_lines} />
        <StatCard label="Code" value={$result.code} />
        <StatCard label="Comments" value={$result.comments} />
      </div>
      <SecretsPanel findings={$findings} />
      <LanguageChart languages={$result.languages} />
      <FileTable files={$result.largest_files} />
    {/if}
  {:else if $loading}
    <p class="hint">Scanning…</p>
  {:else}
    <p class="hint">
      Choose a workspace folder, such as the one that holds your projects,<br />
      to see their size, git status and security checks.
    </p>
  {/if}
</main>

<style>
  main { max-width: 1100px; margin: 0 auto; padding: 28px 24px; display: flex; flex-direction: column; gap: var(--gap); }
  header { display: flex; justify-content: space-between; align-items: center; gap: 16px; }
  h1 { font-size: 1.5rem; }
  .path { color: var(--text-muted); font-family: var(--mono); font-size: 0.85rem; margin-top: 4px; }
  .right { display: flex; align-items: center; gap: 14px; }
  .status { font-size: 0.8rem; color: var(--danger); }
  .status.online { color: var(--ok); }
  .stats { display: grid; grid-template-columns: repeat(auto-fit, minmax(180px, 1fr)); gap: var(--gap); }
  .text-stat { background: var(--surface); border: 1px solid var(--border); border-radius: var(--radius); padding: 16px 20px; display: flex; flex-direction: column; gap: 6px; }
  .text-stat span { color: var(--text-muted); font-size: 0.8rem; text-transform: uppercase; letter-spacing: 0.05em; }
  .text-stat strong { font-size: 1.9rem; }
  .detail-title { margin-top: 12px; font-size: 1.2rem; }
  .hint { color: var(--text-muted); text-align: center; padding: 80px 0; line-height: 1.6; }
  .error { color: var(--danger); }
</style>
