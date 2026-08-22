<script lang="ts">
  let { onclick } = $props();
</script>

<button class="close-btn" {onclick}>&times;</button>

<style>
  .close-btn {
    border: none;
    cursor: pointer;
    color: #888888;
    border-radius: 4px;
    padding: 2px 6px;
    font-size: 14px;
    transition: all 0.15s ease;
  }

  .close-btn:hover {
    background-color: #c72e2e;
    color: #ffffff;
  }
</style>
