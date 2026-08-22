<script lang="ts">
  let {
    value = $bindable(false),
    label = "",
    fontSize = 16,
    design = "slider",
  }: {
    value?: boolean;
    label?: string;
    fontSize?: number;
    design?: string;
  } = $props();
</script>

<label class="switch-wrapper" style="font-size: {fontSize}px" data-design={design}>
  <input type="checkbox" bind:checked={value} />
  <span class="track">
    <span class="thumb"></span>
  </span>
  {#if label}
    <span class="label">{label}</span>
  {/if}
</label>

<style>
  .switch-wrapper {
    display: inline-flex;
    align-items: center;
    gap: 10px;
    cursor: pointer;
    color: #ffffff;
    user-select: none;
  }

  input[type="checkbox"] {
    display: none;
  }

  .track {
    position: relative;
    width: 2.4em;
    height: 1.3em;
    border-radius: 999px;
    background-color: #666666;
    transition: background-color 0.2s ease;
    flex-shrink: 0;
  }

  .thumb {
    position: absolute;
    top: 0.15em;
    left: 0.15em;
    width: 1em;
    height: 1em;
    border-radius: 50%;
    background-color: #ffffff;
    transition: transform 0.2s ease;
  }

  input:checked + .track {
    background-color: #64b5f6;
  }

  input:checked + .track .thumb {
    transform: translateX(1.1em);
  }

  .label {
    font-size: 0.7em;
  }
</style>
