<!-- App.svelte - root component. Lays out the dashboard shell:
     header + project picker, then the stat cards and charts. -->
<script>
  import { onMount } from 'svelte';
  import { connect, pickFolder, scan } from './lib/api.js';
  import { apiOnline, project, result, loading, error } from './lib/stores.js';
  import ProjectPicker from './components/ProjectPicker.svelte';
  import StatCard from './components/StatCard.svelte';
  import LanguageChart from './components/LanguageChart.svelte';
  import FileTable from './components/FileTable.svelte';

  // Runs once when the window opens: find out whether the API is up.
  onMount(async () => {
    $apiOnline = await connect();
  });

  // Picker click: choose a folder, then scan it and store the outcome.
  async function choose() {
    const path = await pickFolder();
    if (!path) return; // dialog cancelled
    $project = path;
    $error = null;
    $loading = true;
    try {
      $result = await scan(path);
    } catch (e) {
      $error = e.message;
    } finally {
      $loading = false;
    }
  }
</script>

<main>
  <header>
    <div>
      <h1>devscope</h1>
      <p class="path">{$project ?? 'No project selected'}</p>
    </div>
    <div class="right">
      <span class="status" class:online={$apiOnline}>{$apiOnline ? 'API online' : 'API offline'}</span>
      <ProjectPicker onpick={choose} disabled={$loading} />
    </div>
  </header>

  {#if $error}
    <p class="error">{$error}</p>
  {/if}

  {#if $loading}
    <p class="hint">Scanning…</p>
  {:else if $result}
    <div class="stats">
      <StatCard label="Files" value={$result.total_files} />
      <StatCard label="Lines" value={$result.total_lines} />
      <StatCard label="Code" value={$result.code} />
      <StatCard label="Comments" value={$result.comments} />
    </div>
    <LanguageChart languages={$result.languages} />
    <FileTable files={$result.largest_files} />
  {:else}
    <p class="hint">Choose a project folder to see its stats.</p>
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
  .hint { color: var(--text-muted); text-align: center; padding: 80px 0; }
  .error { color: var(--danger); }
</style>
