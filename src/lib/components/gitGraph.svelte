<script lang="ts">
  import { createGitgraph } from "@gitgraph/js";
  import { myGitTheme } from "$lib/config/gitTheme";
  import type { CommitInfo } from "$lib/config/GitActionsMenu";
  import CloseButton from "./CloseButton.svelte";

  let {
    activeView,
    commits = [],
  }: {
    activeView: "split" | "actions" | "tree";
    commits: CommitInfo[];
  } = $props();

  let gitgraphElement = $state<HTMLDivElement>();
  let isBarActive = $state(false);

  let selectedCommit = $state<CommitInfo | null>(null);
  //let selectedCommit = $state<string | null>(null);

  function renderGitGraph(commitsList: CommitInfo[]) {
    // on commitInfo pour éviter un conflit avec commits

    if (!gitgraphElement) {
      // si la référence de div est vide (null / undefined), renvoie true
      return; // le return permet d'empêcher le reste de la fonction de s'executer si c'est vrai
    }

    gitgraphElement.innerHTML = ""; // On néttoie la DOM à chaque fois quela fonction renderGitGraph est appelé pour éviter pour déssiner un nouvel arbre git

    const gitgraph = createGitgraph(gitgraphElement, { template: myGitTheme });

    const branches: Record<string, any> = {}; // on créer un objet vide, Record<> indique qu'il s'agit d'un objet, string indique que la clé sera du string, any indique que la valeur peut etre de n'importe quel type
    branches["main"] = gitgraph.branch("main"); // Reçoit le bleu par défaut

    const reversedCommits = [...commitsList].reverse(); // reverse inverse l'ordre des éléments une liste, on le reverse pour déssiner du bas (commit les plus ancien) vers le haut (commit les plus récents)
    const totalCommits = reversedCommits.length;

    const commitsById = new Map(commitsList.map((c) => [c.id, c]));
    reversedCommits.forEach((c, index) => {
      //const branchName = Array.isArray(c.branches) ? c.branches[0] : c.branches;
      const currentBranchName =
        Array.isArray(c.branches) && c.branches.length > 0
          ? c.branches[0]
          : "main";

      if (!branches[currentBranchName]) {
        branches[currentBranchName] =
          branches["main"].branch(currentBranchName);
      }

      const isHead = index === totalCommits - 1;

      const commitOptions = {
        subject: c.message,
        hash: c.id,
        author: c.author,
        tag: isHead ? "HEAD" : undefined,
        onClick: () => openBoxOnCommitClick(c), // on appelle la fonction au clique sur un commit
      };

      const isMerge = Array.isArray(c.parents) && c.parents.length > 1;
      const secondParent = isMerge ? commitsById.get(c.parents[1]) : undefined;
      const sourceBranchName = secondParent?.branches?.[0];

      if (
        isMerge &&
        sourceBranchName &&
        branches[sourceBranchName] &&
        sourceBranchName !== currentBranchName
      ) {
        branches[currentBranchName].merge(
          branches[sourceBranchName],
          commitOptions,
        );
      } else {
        branches[currentBranchName].commit(commitOptions);
      }
    });
  }

  function openBoxOnCommitClick(commit: CommitInfo) {
    console.log("Commit cliqué :", commit); // commit est un objet
    isBarActive = true;
    selectedCommit = commit;
  }

  $effect(() => {
    console.log("Commits reçus :", $state.snapshot(commits));
    renderGitGraph(commits);
  });
</script>

<div
  class="tree-wrapper"
  class:hidden={activeView === "actions"}
  class:full-width={activeView === "tree"}
>
  <div bind:this={gitgraphElement}></div>

  {#if isBarActive === true}
    <div class="bar">
      <CloseButton onclick={() => (isBarActive = false)} />
      <h1>info commit</h1>
      <p>id: {selectedCommit?.id}</p>
      <p>message: {selectedCommit?.message}</p>
      <p>parent : {selectedCommit?.parents}</p>
      <p>auteur : {selectedCommit?.author}</p>
    </div>
  {/if}
</div>

<style>
  .tree-wrapper {
    background-color: #2a2a2a;
    border: 2px dashed #666666;
    flex: 1;
    color: #aaaaaa;
    font-family: "Inter", sans-serif;
    font-size: 1.2rem;
    font-weight: bold;
    border-radius: 6px;
    box-sizing: border-box;

    display: flex;
    flex-direction: row;

    align-items: flex-start;
    justify-content: center;
    overflow: auto;
    opacity: 1;

    transition:
      flex 0.4s cubic-bezier(0.4, 0, 0.2, 1),
      opacity 0.25s ease-in-out,
      padding 0.4s ease;
  }
  .bar {
    background-color: rgb(65, 65, 65);
    border: none;
    border-radius: 10px;
    padding: 10px;
  }

  .tree-wrapper :global(svg) {
    padding: 60px;
  }

  .tree-wrapper :global(svg circle) {
    cursor: pointer;
  }

  .tree-wrapper:active {
    cursor: grabbing;
  }

  .hidden {
    flex: 0;
    width: 0;
    padding: 0;
    opacity: 0;
    pointer-events: none;
    border: none;
  }
</style>
