<!-- ProjectPicker.svelte - button/drop zone to choose a project folder via the preload bridge. -->
<script>
  // `onpick` is a callback the parent passes in; we call it when clicked.
  // `$props()` is how Svelte 5 components receive inputs.
  let { onpick, disabled = false, label = 'Choose folder…' } = $props();
</script>

<button class="picker" {disabled} onclick={onpick}>{label}</button>

<style>
  .picker {
    background: var(--accent);
    color: #0d1117;
    border: 0;
    border-radius: var(--radius);
    padding: 10px 18px;
    font-weight: 600;
  }
  .picker:disabled { opacity: 0.5; cursor: progress; }
</style>
