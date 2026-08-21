<script lang="ts">
  import { createGitgraph } from "@gitgraph/js";
  import { myGitTheme } from "$lib/config/gitTheme";
  import type { CommitInfo } from "$lib/config/GitActionsMenu";
  import CloseButton from "../CloseButton.svelte";

  import { CommitInteractionState } from "./CommitInteractionState.svelte";
  import CommitDetailPanel from "./CommitDetailsPanel.svelte";
  import ComparaisonPanel from "./CommitComparaisonPanel.svelte";

  let {
    activeView,
    commits = [],
    path,
  }: {
    activeView: "split" | "actions" | "tree";
    commits: CommitInfo[];
    path: string | null;
  } = $props();

  // beaucoup de variable qui sont déclaré mais chacun ont un rôle à joué

  const commitState = new CommitInteractionState(); // le new ne s'utilisque pour instancier des classes TypeScript
  //const commitDetail = ActionPanel();

  let gitgraphElement = $state<HTMLDivElement>();
  let viewportElement = $state<HTMLDivElement>();

  let panX = $state(0); // c'est le décalage en x entre le <div> de canvas-viewport et le <div> canvas-world
  let panY = $state(0); // pareil mais en y
  let zoom = $state(1);
  let isDragging = $state(false);
  let dragStartX = $state(0);
  let dragStartY = $state(0);

  function handleMouseDown(e: MouseEvent) {
    if ((e.target as HTMLElement).closest(".panel-card, circle, button"))
      return;
    isDragging = true;
    dragStartX = e.clientX - panX;
    dragStartY = e.clientY - panY;
  }

  function handleMouseMove(e: MouseEvent) {
    if (!isDragging) return;
    panX = e.clientX - dragStartX;
    panY = e.clientY - dragStartY;
  }

  function handleMouseUp() {
    isDragging = false;
  }

  function handleWheel(e: WheelEvent) {
    e.preventDefault();
    const zoomFactor = 1.1; // chaque coup de mollette équivaut à une variation de 10%
    // le zoom minimum et maximum correspond à résultat en pourcentage soit 30% et 300%
    const minZoom = 0.3;
    const maxZoom = 3.0;

    const rect = viewportElement?.getBoundingClientRect();
    if (!rect) return;

    // ce les coordonnées dans le canvas
    const mouseX = e.clientX - rect.left;
    const mouseY = e.clientY - rect.top;

    const newZoom =
      e.deltaY < 0
        ? Math.min(zoom * zoomFactor, maxZoom)
        : Math.max(zoom / zoomFactor, minZoom);

    panX = mouseX - ((mouseX - panX) / zoom) * newZoom;
    panY = mouseY - ((mouseY - panY) / zoom) * newZoom;
    zoom = newZoom;
  }

  function resetView() {
    panX = 50;
    panY = 50;
    zoom = 1;
  }

  function renderGitGraph(commitsList: CommitInfo[]) {
    if (!gitgraphElement) {
      // si la référence de div est vide (null / undefined), renvoie true
      return; // le return permet d'empêcher le reste de la fonction de s'executer si c'est vrai
    }

    const gitgraph = createGitgraph(gitgraphElement, { template: myGitTheme });
    type BranchType = ReturnType<typeof gitgraph.branch>;
    const branches: Record<string, BranchType> = {};
    // on créer un objet vide, Record<> indique qu'il s'agit d'un objet, string indique que la clé sera du string, any indique que la valeur peut etre de n'importe quel type
    // note : on utilisait any avant mais j'ai finis par mêttre  BranchType à la place qu'on définit juste au dessus
    // le github action voulait qu'on utilisait une variable typé hors any ne l'est pas
    // on déclare BranchType avec le type renvoyé par la fonction .branch()
    // les clé sont en string, les valeur auront le type objet

    branches["main"] = gitgraph.branch("main"); // Reçoit le bleu par défaut

    const reversedCommits = [...commitsList].reverse();
    // on vient modifier l'ordre des commits SANS modifier le tableau d'origine grâce à [...comitsList]
    // on vient changer l'ordre des commit de cette copie grâce à .reverse();
    // ==> reversedCommid à l'indice 0 correspond au commit le plus ancien
    // on a besoin de reversedCommit pour déssiner les commit du plus ancien au plus récent
    // gitgraph/js ne sait pas déssiner un commit parent à partir de l'enfant

    //const totalCommits = reversedCommits.length;
    const commitsById = new Map(commitsList.map((c) => [c.id, c]));

    reversedCommits.forEach((c) => {
      const currentBranchName =
        Array.isArray(c.branches) && c.branches.length > 0
          ? c.branches[0]
          : "main";

      if (!branches[currentBranchName]) {
        branches[currentBranchName] =
          branches["main"].branch(currentBranchName);
      }

      //const isHead = index === totalCommits - 1;
      const isHead = c.is_head;

      const commitOptions = {
        subject: c.message,
        hash: c.id,
        author: c.author,
        tag: isHead ? "HEAD" : undefined,
        onClick: () => commitState.openBoxOnCommitClick(c, path),
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
          commitOptions.subject, // utilisation du paramètre subject à l'objet coomitOptions
        );
      } else {
        branches[currentBranchName].commit(commitOptions);
      }
    });
  }

  $effect(() => {
    // il execute la fonction renderGitGraph(commits) et le réexcute automaitiquement des qu'une vairable
    // réactive change dans la fonction renderGitGraph()
    renderGitGraph(commits);
  });
</script>

<svelte:window onmouseup={handleMouseUp} onmousemove={handleMouseMove} />

<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
<div
  bind:this={viewportElement}
  class="canvas-viewport"
  class:dragging={isDragging}
  class:hidden={activeView === "actions"}
  class:full-width={activeView === "tree"}
  onmousedown={handleMouseDown}
  onwheel={handleWheel}
  role="region"
  aria-label="Git Graph Viewport"
>
  <div
    class="canvas-world"
    style="transform: translate3d({panX}px, {panY}px, 0) scale({zoom});"
  >
    {#key commits}
      <!-- "Utilise la variable commits comme clé d'identification de tout ce qui se trouve entre {#key} et {/key}.
      Tant que cette clé ne change pas, laisse le <div> tranquille. 
      Mais dès que la valeur ou la référence de commits change, considère que ce bloc est périmé : 
      détruis-le entièrement et reconstruis-le à neuf." 
      
      --gemini j'ai trouvé que son explication était bien meilleur que le miens, 

      ici il permet de redéssiner le gitgraph, il permet d'éviter le warning obtenu avec le github action lorsqu'on utilisait ==> gitgraphElement.textContent = ""-->
      <div bind:this={gitgraphElement}></div>
    {/key}
  </div>

  <div class="zoom-controls">
    <button onclick={() => (zoom = Math.min(zoom * 1.15, 3.0))}>+</button>
    <span>{Math.round(zoom * 100)}%</span>
    <button onclick={() => (zoom = Math.max(zoom / 1.15, 0.3))}>−</button>
    <button onclick={resetView} title="Recentrer">⟲</button>
  </div>

  <!-- notre menu d'actions sur le noeud, on le met comme enfant du <div> canvas-viewport -->
  <!-- cela permet à ce qu'il reste statique contrairement au <div> canvas-world -->
  {#if commitState.isBarActive}
    <div class="panel-card">
      <div class="panel-header">
        <div class="header-title">
          <span class="commit-badge"
            >{commitState.selectedCommit?.id.slice(0, 7)}</span
          >
          <h3>Actions Commit</h3>
        </div>
        <CloseButton onclick={() => (commitState.isBarActive = false)} />
      </div>

      <p class="commit-summary">
        {commitState.selectedCommit?.message}
      </p>

      <div class="action-buttons">
        <button
          class="action-btn"
          onclick={() => commitState.displayCommitInfo()}
        >
          <div class="btn-text">
            <span class="btn-title">Détails du commit</span>
            <span class="btn-sub">Auteur, parents et hash complet</span>
          </div>
        </button>

        <button
          class="action-btn"
          onclick={() => commitState.startComparaison()}
        >
          <div class="btn-text">
            <span class="btn-title">Comparer avec...</span>
            <span class="btn-sub">Sélectionner un 2ᵉ commit</span>
          </div>
        </button>
      </div>
    </div>
  {:else if commitState.commitInfoDisplayed}
    <CommitDetailPanel state={commitState} />
  {:else if commitState.isComparaisonActive}
    <ComparaisonPanel state={commitState} />
  {/if}
</div>

<style>
  .canvas-viewport {
    position: relative;
    width: 100%;
    height: 100%;
    flex: 1;
    background-color: #1a1a1e;
    background-image: radial-gradient(#33333d 1px, transparent 1px);

    /* on créer des cercle miniature de 1px de rayon*/

    /* transparant permet à évtier que nos cercle se touchent pour former un bloc d'une couleur uni
    l'utilisaiton de transparant 1px permet de laisser le fond avec la couleur du background-color */
    background-size: 24px 24px;
    overflow: hidden;
    cursor: grab;
    user-select: none;
    box-sizing: border-box;
    transition:
      flex 0.4s cubic-bezier(0.4, 0, 0.2, 1),
      opacity 0.25s ease-in-out;
  }

  .canvas-viewport.dragging {
    cursor: grabbing;
  }

  .canvas-world {
    position: absolute;
    top: 0;
    left: 0;
    transform-origin: 0 0;
    will-change: transform;
    pointer-events: auto;
  }

  .canvas-world :global(svg) {
    overflow: visible;
    padding: 40px;
  }

  .canvas-world :global(svg circle) {
    cursor: pointer;
  }

  .zoom-controls {
    position: absolute;
    bottom: 16px;
    left: 16px;
    display: flex;
    align-items: center;
    gap: 6px;
    background: rgba(30, 30, 36, 0.85);
    backdrop-filter: blur(8px);
    border: 1px solid #3f3f46;
    border-radius: 6px;
    padding: 4px 8px;
    color: #a1a1aa;
    font-family: monospace;
    font-size: 0.75rem;
    z-index: 5;
  }

  .zoom-controls button {
    background: #27272a;
    color: #f4f4f5;
    border: 1px solid #3f3f46;
    border-radius: 4px;
    width: 24px;
    height: 24px;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    font-size: 0.85rem;
    transition: background 0.15s;
  }

  .zoom-controls button:hover {
    background: #3f3f46;
  }

  /* necessaire pour le css du clique sur un noeud pour afficher le menu d'intéraction */
  .panel-card {
    position: absolute;
    top: 16px;
    right: 16px;
    z-index: 10;
    width: 280px;
    background: #1e1e24;
    border: 1px solid #3f3f46;
    border-radius: 8px;
    padding: 14px;
    box-shadow:
      0 10px 25px -5px rgba(0, 0, 0, 0.5),
      0 8px 10px -6px rgba(0, 0, 0, 0.5);
    display: flex;
    flex-direction: column;
    gap: 12px;
    animation: slideIn 0.15s ease-out;
  }

  @keyframes slideIn {
    from {
      opacity: 0;
      transform: translateY(-6px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

  .panel-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    border-bottom: 1px solid #2e2e36;
    padding-bottom: 8px;
  }

  .header-title {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .header-title h3 {
    margin: 0;
    font-size: 0.95rem;
    font-weight: 600;
    color: #f4f4f5;
  }

  .commit-badge {
    background: #27272a;
    color: #60a5fa;
    font-family: monospace;
    font-size: 0.75rem;
    padding: 2px 6px;
    border-radius: 4px;
    border: 1px solid #3b82f6;
  }

  .commit-summary {
    margin: 0;
    font-size: 0.82rem;
    color: #a1a1aa;
    font-weight: normal;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    min-width: 0;
  }

  .action-buttons {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .action-btn {
    display: flex;
    align-items: center;
    gap: 10px;
    background: #27272a;
    border: 1px solid #3f3f46;
    color: #e4e4e7;
    padding: 8px 10px;
    border-radius: 6px;
    cursor: pointer;
    text-align: left;
    transition:
      background-color 0.15s ease,
      border-color 0.15s ease,
      transform 0.05s ease;
  }

  .action-btn:hover {
    background: #323238;
    border-color: #52525b;
  }

  .action-btn:active {
    transform: scale(0.98);
  }

  .btn-text {
    display: flex;
    flex-direction: column;
    gap: 1px;
  }

  .btn-title {
    font-size: 0.85rem;
    font-weight: 600;
    color: #fafafa;
  }

  .btn-sub {
    font-size: 0.7rem;
    color: #71717a;
    font-weight: normal;
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
