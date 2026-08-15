<script lang="ts">
  import { createGitgraph } from "@gitgraph/js";
  import { myGitTheme } from "$lib/config/gitTheme";
  import type { CommitInfo } from "$lib/config/GitActionsMenu";
  import CloseButton from "./CloseButton.svelte";
  import { invoke } from "@tauri-apps/api/core";

  let {
    activeView,
    commits = [],
    path,
  }: {
    activeView: "split" | "actions" | "tree";
    commits: CommitInfo[];
    path: string | null;
  } = $props();

  let gitgraphElement = $state<HTMLDivElement>();
  let isBarActive = $state(false);
  let commitInfoDisplayed = $state(false);
  let isComparaisonActive = $state(false);
  let diffResult = $state<string | null>(null);
  let diffError = $state<string | null>(null);

  let firstHash = $state<string | null>(null);
  let secondHash = $state<string | null>(null);

  let selectedCommit = $state<CommitInfo | null>(null);

  function renderGitGraph(commitsList: CommitInfo[]) {
    // on commitInfo pour éviter un conflit avec commits

    if (!gitgraphElement) {
      // si la référence de div est vide (null / undefined), renvoie true
      return; // le return permet d'empêcher le reste de la fonction de s'executer si c'est vrai
    }

    //gitgraphElement.innerHTML = ""; // On néttoie la DOM à chaque fois quela fonction renderGitGraph est appelé pour éviter pour déssiner un nouvel arbre git
    gitgraphElement.textContent = "";

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
    if (isComparaisonActive) {
      // si on a appyer sur le bouton pour comparer
      if (commit.id === firstHash) {
        // si le hash du commit qu'on selectionne est pareil que le premier
        diffError = "Choisis un commit différent du premier.";
        return;
      }

      secondHash = commit.id;
      //isComparaisonActive = false;
      commitComparaison();
      return;
    }

    selectedCommit = commit;
    isBarActive = true;
  }

  function displayCommitInfo() {
    commitInfoDisplayed = true;
    isBarActive = false; // pour cacher le menu de bouton
  }

  function startComparaison() {
    if (!selectedCommit) return; // si selectedCommit vaut null alors on renvoie vrai
    // si selectedCommit est un objet renvoit false
    // on return si c'est false pour arrêter toute de suite l'execution de la fonction

    firstHash = selectedCommit.id; // on récupère le hash du commit déja selectionné (lorsqu'on a cliquer pour faire apparaitre le menu)
    secondHash = null; // on set toute les valeur à null
    diffResult = null;
    diffError = null;
    isBarActive = false; // on ferme le menu d'intéraction
    isComparaisonActive = true; // on ouvre le conteneur de comparaison
  }

  function cancelComparaison() {
    isComparaisonActive = false;
    firstHash = null;
    secondHash = null;
  }

  async function commitComparaison() {
    if (firstHash === null || secondHash === null || path === null) {
      return;
    }

    diffError = null;
    try {
      diffResult = await invoke<string>("compare_commit", {
        path,
        oldCommit: firstHash,
        newCommit: secondHash,
      });
    } catch (err) {
      diffError = String(err);
    }
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
      <h1>menu d'intéraction</h1>
      <button onclick={() => displayCommitInfo()}>voir détaille commit</button>
      <button onclick={startComparaison}>comparer avec ...</button>
    </div>
  {:else if commitInfoDisplayed === true}
    <div class="commit-info">
      <CloseButton onclick={() => (commitInfoDisplayed = false)} />
      <h1>info commit</h1>
      <p>id: {selectedCommit?.id}</p>
      <p>message: {selectedCommit?.message}</p>
      <p>parent : {selectedCommit?.parents}</p>
      <p>auteur : {selectedCommit?.author}</p>
    </div>
  {:else if isComparaisonActive === true}
    <div class="commit-diff">
      <CloseButton onclick={cancelComparaison} />
      <p>1er commit sélectionné : {firstHash}</p>

      {#if secondHash === null}
        <p>Sélectionne le 2e commit à comparer...</p>
      {:else if diffResult}
        <!-- si diffResult n'est pas une chaine vide ou null -->
        <p>2e commit sélectionné : {secondHash}</p>
        <p>{diffResult}</p>
        <!-- si diffResult est une chaine vide (j'ai beaucoup galéré sur celui la ...) -->
      {:else if diffResult === ""}
        <p>2e commit sélectionné : {secondHash}</p>
        <p>il n'y a aucune différence</p>
      {:else if diffError}
        <p>2e commit sélectionné : {secondHash}</p>
        <p>quelque chose s'est mal passé</p>
      {/if}
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
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  .bar,
  .commit-info,
  .commit-diff {
    background-color: rgb(65, 65, 65);
    border: none;
    border-radius: 10px;
    padding: 10px;
  }

  .bar button {
    background-color: #2c539e;
    color: white;
    border: none;
    padding: 8px 12px;
    border-radius: 5px;
    cursor: pointer;
    text-align: left;
    transition: background-color 0.2s;
  }

  .bar button:hover {
    background-color: #3b69c4;
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
