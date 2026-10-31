<!-- App.svelte - root component. Lays out the dashboard shell: header with the
     folder picker, tabs, and either the folder overview or the projects view. -->
<script>
  import { onMount } from 'svelte';
  import { connect, pickFolder, getKnownFolders, folder, duplicates, workspace, scan, secrets } from './lib/api.js';
  import {
    apiOnline, known, root, report, dupes, dupesBusy, tab,
    projects, selected, result, findings, loading, error,
  } from './lib/stores.js';
  import ProjectPicker from './components/ProjectPicker.svelte';
  import Tabs from './components/Tabs.svelte';
  import FolderView from './components/FolderView.svelte';
  import ProjectsView from './components/ProjectsView.svelte';

  // Runs once when the window opens: find out whether the API is up, and which OS folders exist.
  onMount(async () => {
    $apiOnline = await connect();
    $known = await getKnownFolders();
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

  // If the user starts a new analysis while one is running, the older one's results are dropped.
  let latest = 0;

  // Analyse a folder. If it holds several code projects, open the Projects tab; otherwise the overview.
  async function analyse(path) {
    if (!path) return; // dialog cancelled
    const id = ++latest;
    $root = path;
    $report = null;
    $dupes = null;
    $projects = [];
    $selected = null;
    $result = null;
    $findings = [];
    await run(async () => {
      const r = await folder(path);
      if (id !== latest) return;
      $report = r;
      $tab = r.project_count >= 2 ? 'projects' : 'folder';
      if ($tab === 'projects') {
        const found = await workspace(path);
        if (id === latest) $projects = found;
      }
    });
  }

  const choose = async () => analyse(await pickFolder());

  // Projects are only measured the first time their tab is opened (it is the slow part).
  async function showTab(id) {
    $tab = id;
    if (id === 'projects' && !$projects.length && $report?.project_count > 0) {
      await run(async () => {
        $projects = await workspace($root);
      });
    }
  }

  // Row click in the projects list: load line stats and the secrets check in parallel.
  async function inspect(project) {
    $selected = project;
    await run(async () => {
      [$result, $findings] = await Promise.all([scan(project.path), secrets(project.path)]);
    });
  }

  // Duplicate search reads file contents, so it only runs when asked.
  async function findDuplicates() {
    $dupesBusy = true;
    $error = null;
    try {
      $dupes = await duplicates($root);
    } catch (e) {
      $error = e.message;
    } finally {
      $dupesBusy = false;
    }
  }

  const quick = [
    ['downloads', 'Downloads'],
    ['documents', 'Documents'],
    ['desktop', 'Desktop'],
  ];

  let tabs = $derived([
    { id: 'folder', label: 'Folder overview' },
    ...($report?.project_count > 0 ? [{ id: 'projects', label: 'Code projects', count: $report.project_count }] : []),
  ]);
</script>

{#if $loading}<div class="progress"></div>{/if}

<main>
  <header>
    <div class="brand">
      <span class="mark"></span>
      <div>
        <h1>devscope</h1>
        <p class="path" title={$root ?? ''}>{$root ?? 'See what is inside any folder'}</p>
      </div>
    </div>
    <div class="right">
      <span class="status" class:online={$apiOnline}><i></i>{$apiOnline ? 'Engine ready' : 'Engine offline'}</span>
      <ProjectPicker onpick={choose} disabled={$loading || !$apiOnline} label="Choose folder…" />
    </div>
  </header>

  {#if $error}
    <div class="error" role="alert">{$error}</div>
  {/if}

  {#if $report}
    <Tabs items={tabs} active={$tab} onchange={showTab} />

    {#if $tab === 'projects'}
      {#if $projects.length}
        <ProjectsView projects={$projects} selected={$selected} result={$result} findings={$findings} onselect={inspect} />
      {:else if $loading}
        <p class="hint">Measuring projects…</p>
      {/if}
    {:else}
      <FolderView report={$report} dupes={$dupes} dupesBusy={$dupesBusy} onfinddupes={findDuplicates} />
    {/if}
  {:else if $loading}
    <div class="hero">
      <span class="spinner"></span>
      <p>Analysing <strong>{$root}</strong>…</p>
    </div>
  {:else}
    <div class="hero">
      <h2>Pick a folder to explore</h2>
      <p>
        See what takes up space, what is old or duplicated, and, for code folders, which projects have
        unsaved work or leaked secrets. Everything stays on your computer and nothing is changed.
      </p>
      {#if $apiOnline}
        <div class="quick">
          {#each quick as [key, name]}
            {#if $known[key]}
              <button onclick={() => analyse($known[key])}>{name}</button>
            {/if}
          {/each}
        </div>
      {:else}
        <p class="offline">The analysis engine is not running, so folders cannot be analysed yet.</p>
      {/if}
    </div>
  {/if}
</main>

<style>
  main { max-width: 1120px; margin: 0 auto; padding: 24px 24px 56px; display: flex; flex-direction: column; gap: var(--gap); }

  header { display: flex; justify-content: space-between; align-items: center; gap: 16px; flex-wrap: wrap; padding-bottom: 6px; }
  .brand { display: flex; align-items: center; gap: 14px; min-width: 0; }
  .mark { width: 38px; height: 38px; border-radius: 11px; background: var(--accent-gradient); position: relative; flex: none; box-shadow: 0 6px 18px rgba(91, 140, 255, 0.35); }
  /* a small "scope" ring and dot */
  .mark::before { content: ''; position: absolute; inset: 9px; border: 2.5px solid #fff; border-radius: 50%; }
  .mark::after { content: ''; position: absolute; inset: 16px; background: #fff; border-radius: 50%; }
  h1 { font-size: 1.3rem; letter-spacing: -0.01em; }
  .path { color: var(--text-muted); font-family: var(--mono); font-size: 0.78rem; margin-top: 2px; max-width: 520px; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
  .right { display: flex; align-items: center; gap: 16px; }

  .status { display: inline-flex; align-items: center; gap: 7px; font-size: 0.78rem; color: var(--danger); }
  .status i { width: 8px; height: 8px; border-radius: 50%; background: currentColor; box-shadow: 0 0 0 3px color-mix(in srgb, currentColor 22%, transparent); }
  .status.online { color: var(--ok); }

  .progress { position: fixed; inset: 0 0 auto 0; height: 3px; background: var(--accent-gradient); animation: slide 1.1s ease-in-out infinite; transform-origin: left; z-index: 10; }
  @keyframes slide { 0% { transform: scaleX(0.05); } 50% { transform: scaleX(0.7); } 100% { transform: scaleX(1); opacity: 0.2; } }

  .error { background: color-mix(in srgb, var(--danger) 12%, transparent); border: 1px solid color-mix(in srgb, var(--danger) 40%, transparent); color: var(--danger); border-radius: var(--radius-sm); padding: 10px 14px; font-size: 0.9rem; }

  .hero { text-align: center; padding: 96px 20px; display: flex; flex-direction: column; align-items: center; gap: 14px; }
  .hero h2 { font-size: 1.6rem; letter-spacing: -0.01em; }
  .hero p { color: var(--text-muted); max-width: 520px; line-height: 1.6; }
  .hero .offline { color: var(--danger); }
  .quick { display: flex; gap: 10px; margin-top: 8px; flex-wrap: wrap; justify-content: center; }
  .quick button { background: var(--surface); border: 1px solid var(--border); border-radius: var(--radius-sm); padding: 9px 18px; font-weight: 600; transition: border-color 0.12s ease, transform 0.12s ease; }
  .quick button:hover { border-color: var(--accent); transform: translateY(-1px); }
  .spinner { width: 28px; height: 28px; border: 3px solid var(--border); border-top-color: var(--accent); border-radius: 50%; animation: spin 0.8s linear infinite; }
  .hint { color: var(--text-muted); text-align: center; padding: 60px 0; }
</style>
