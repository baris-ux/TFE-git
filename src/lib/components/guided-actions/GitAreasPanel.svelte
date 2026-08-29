<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  let result = $state<string>("");
  let notPushed = $state<string>("");

  let { path, refreshGitAreaPanelCount } = $props<{
    path: string | null;
    refreshGitAreaPanelCount?: number;
  }>();

  // $props() permet de récupéré des valeurs mais pas de les modifier

  let workingArea = $derived(
    result
      .split("\n")
      .filter((line) => line.length > 0)
      .filter(
        (item) =>
          (item[0] === "?" && item[1] === "?") ||
          item[1] === "M" ||
          item[1] === "D",
      ),
  );

  let staggingArea = $derived(
    result
      .split("\n")
      .filter((line) => line.length > 0)
      .filter((item) => item[0] === "A" || item[0] === "M"),
  );

  let unpushedCommits = $derived(
    notPushed
      .split("\n")
      .map((line) => line.trim())
      .filter((line) => line.length > 0),
  );

  async function getActifFiles() {
    result = await invoke<string>("get_actif_files", { path });
  }

  async function getNotPushedCommits() {
    notPushed = await invoke<string>("get_not_pushed_commits", { path });
  }

  $effect(() => {
    refreshGitAreaPanelCount;

    if (path) {
      getActifFiles();
      getNotPushedCommits();
    }
  });
</script>

<div class="git-columns">
  <div class="column">
    <h3>Working Area ({workingArea.length})</h3>
    {#each workingArea as item}
      <p>{item}</p>
    {/each}
  </div>

  <div class="column">
    <h3>Stagging Area</h3>
    {#each staggingArea as item}
      <p>{item}</p>
    {/each}
  </div>

  <div class="column">
    <h3>Local Repository</h3>
    {#if unpushedCommits.length === 0}
      <p>Synchronisé, aucun push en attente</p>
    {:else}
      {#each unpushedCommits as item}
        <p>{item}</p>
      {/each}
    {/if}
  </div>
</div>

<style>
  .git-columns {
    display: flex;
    flex-direction: row;
    gap: 15px;
  }

  .column {
    flex: 1;
    background-color: #2b2b2b;
    border-radius: 6px;
    padding: 12px;
    gap: 8px;
    border-top: 3px solid #666;
  }

  .column:nth-child(1) {
    border-top-color: #f87171;
  }

  .column:nth-child(2) {
    border-top-color: #4ade80;
  }

  .column:nth-child(3) {
    border-top-color: #60a5fa;
  }

  .column p {
    margin: 0;
    padding: 6px 8px;
    background-color: #1f1f1f;
    border-radius: 4px;
    font-family: monospace;
    font-size: 0.8rem;
    color: #e0e0e0;
    word-break: break-all;
  }
</style>
