<!-- ProjectPicker.svelte - button to choose a folder via the preload bridge. -->
<script>
  // `onpick` is a callback the parent passes in; we call it when clicked.
  // `$props()` is how Svelte 5 components receive inputs.
  let { onpick, disabled = false, label = 'Choose folder…' } = $props();
</script>

<button class="picker" {disabled} onclick={onpick}>{label}</button>

<style>
  .picker {
    background: var(--accent-gradient);
    color: #fff;
    border: 0;
    border-radius: var(--radius-sm);
    padding: 10px 20px;
    font-weight: 650;
    box-shadow: 0 6px 18px rgba(91, 140, 255, 0.28);
    transition: transform 0.12s ease, box-shadow 0.12s ease;
  }
  .picker:hover:not(:disabled) { transform: translateY(-1px); box-shadow: 0 8px 22px rgba(91, 140, 255, 0.4); }
  .picker:disabled { opacity: 0.5; cursor: not-allowed; }
</style>
