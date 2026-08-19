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

  // beaucoup de variable qui sont déclaré mais chacun ont un rôle à joué

  let gitgraphElement = $state<HTMLDivElement>();
  let viewportElement = $state<HTMLDivElement>();

  let panX = $state(0); // c'est le décalage en x entre le <div> de canvas-viewport et le <div> canvas-world
  let panY = $state(0); // pareil mais en y
  let zoom = $state(1);
  let isDragging = $state(false);
  let dragStartX = $state(0);
  let dragStartY = $state(0);

  let isBarActive = $state(false); // c'est le menu d'intéraction qu'on met à faux par défaut pour qu'il ne soit pas affiché
  let commitInfoDisplayed = $state(false);
  let isComparaisonActive = $state(false);
  let diffResult = $state<string | null>(null);
  let diffError = $state<string | null>(null);
  let firstHash = $state<string | null>(null);
  let secondHash = $state<string | null>(null);
  let selectedCommit = $state<CommitInfo | null>(null); // cette variable va contenir l'objet CommitInfo qu'on a définit dans GitActionMenu.ts
  // il va stocker les différent information du noeud sur lequel on va cliquer sur un noeud

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

    //gitgraphElement.innerHTML = ""; // On néttoie la DOM à chaque fois quela fonction renderGitGraph est appelé pour éviter pour déssiner un nouvel arbre git
    //gitgraphElement.textContent = "";

    const gitgraph = createGitgraph(gitgraphElement, { template: myGitTheme });
    const branches: Record<string, any> = {}; // on créer un objet vide, Record<> indique qu'il s'agit d'un objet, string indique que la clé sera du string, any indique que la valeur peut etre de n'importe quel type
    branches["main"] = gitgraph.branch("main"); // Reçoit le bleu par défaut

    const reversedCommits = [...commitsList].reverse();
    // on vient modifier l'ordre des commits SANS modifier le tableau d'origine grâce à [...comitsList]
    // on vient changer l'ordre des commit de cette copie grâce à .reverse();
    // on reverse car on doit déssiner les commits du plus récent vers le plus ancien
    // cela est du au fait qu'un commit connait le hash de son parent mais l'inverse n'est pas vrai

    const totalCommits = reversedCommits.length;
    const commitsById = new Map(commitsList.map((c) => [c.id, c]));

    reversedCommits.forEach((c, index) => {
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
        onClick: () => openBoxOnCommitClick(c),
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
    // on passe en paramètre l'objet CommitInfo définit dans le config/GitActionsMenu.ts
    if (isComparaisonActive) {
      if (commit.id === firstHash) {
        diffError = "Choisis un commit différent du premier.";
        return;
      }
      secondHash = commit.id;
      commitComparaison();
      return;
    }

    selectedCommit = commit;
    commitInfoDisplayed = false;
    isBarActive = true;
  }

  function displayCommitInfo() {
    commitInfoDisplayed = true;
    isBarActive = false;
  }

  function startComparaison() {
    if (!selectedCommit) return;
    // si selectedCommit est un objet renvoit false
    // si c'est false on return  pour arrêter toute de suite l'execution de la fonction
    firstHash = selectedCommit.id; // on récupère le hash du commit déja selectionné (lorsqu'on a cliquer pour faire apparaitre le menu)
    secondHash = null; // on set toute les valeur à null
    diffResult = null;
    diffError = null;
    isBarActive = false; // on ferme le menu d'intéraction
    isComparaisonActive = true;
  }

  function cancelComparaison() {
    isComparaisonActive = false;
    firstHash = null;
    secondHash = null;
  }

  async function commitComparaison() {
    if (firstHash === null || secondHash === null || path === null) return;
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
  {#if isBarActive}
    <div class="panel-card">
      <div class="panel-header">
        <div class="header-title">
          <span class="commit-badge">{selectedCommit?.id.slice(0, 7)}</span>
          <h3>Actions Commit</h3>
        </div>
        <CloseButton onclick={() => (isBarActive = false)} />
      </div>

      <p class="commit-summary">{selectedCommit?.message}</p>

      <div class="action-buttons">
        <button class="action-btn" onclick={displayCommitInfo}>
          <div class="btn-text">
            <span class="btn-title">Détails du commit</span>
            <span class="btn-sub">Auteur, parents et hash complet</span>
          </div>
        </button>

        <button class="action-btn" onclick={startComparaison}>
          <div class="btn-text">
            <span class="btn-title">Comparer avec...</span>
            <span class="btn-sub">Sélectionner un 2ᵉ commit</span>
          </div>
        </button>
      </div>
    </div>
  {:else if commitInfoDisplayed}
    <div class="panel-card">
      <div class="panel-header">
        <div class="header-title">
          <span class="commit-badge">{selectedCommit?.id.slice(0, 7)}</span>
          <h3>Détails du commit</h3>
        </div>
        <CloseButton onclick={() => (commitInfoDisplayed = false)} />
      </div>

      <div class="details-list">
        <div class="detail-item">
          <span class="detail-label">Message</span>
          <p class="detail-value message">{selectedCommit?.message}</p>
        </div>
        <div class="detail-item">
          <span class="detail-label">Auteur</span>
          <p class="detail-value">{selectedCommit?.author}</p>
        </div>
        <div class="detail-item">
          <span class="detail-label">Hash complet</span>
          <code class="detail-value hash">{selectedCommit?.id}</code>
        </div>
        {#if selectedCommit?.parents?.length}
          <div class="detail-item">
            <span class="detail-label">Parent(s)</span>
            <code class="detail-value hash"
              >{selectedCommit.parents.join(", ")}</code
            >
          </div>
        {/if}
      </div>
    </div>
  {:else if isComparaisonActive}
    <div class="panel-card diff-card">
      <div class="panel-header">
        <div class="header-title">
          <h3>Comparaison</h3>
        </div>
        <CloseButton onclick={cancelComparaison} />
      </div>

      <div class="diff-hashes">
        <div class="hash-tag">
          <span>Base :</span>
          <code>{firstHash?.slice(0, 7)}</code>
        </div>
        <span class="arrow">➔</span>
        <div class="hash-tag">
          <span>Cible :</span>
          <code>{secondHash ? secondHash.slice(0, 7) : "..."}</code>
        </div>
      </div>

      {#if secondHash === null}
        <p class="diff-hint">Cliquez sur un second commit dans le graphe...</p>
      {:else if diffResult}
        <pre class="diff-output">{diffResult}</pre>
        <!-- si diffResult est une chaine vide (j'ai beaucoup galéré sur celui la ...) -->
      {:else if diffResult === ""}
        <p class="diff-empty">Aucune différence détectée entre ces commits.</p>
      {:else if diffError}
        <p class="diff-error">{diffError}</p>
      {/if}
    </div>
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

  /* Panneau Flottant (Fixé dans le coin haut-droit) */
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

  .diff-card {
    width: 340px;
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

  .details-list {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .detail-item {
    display: flex;
    flex-direction: column;
    gap: 2px;
    background: #27272a;
    padding: 6px 8px;
    border-radius: 4px;
  }

  .detail-label {
    font-size: 0.68rem;
    text-transform: uppercase;
    letter-spacing: 0.5px;
    color: #71717a;
    font-weight: 600;
  }

  .detail-value {
    margin: 0;
    font-size: 0.8rem;
    color: #e4e4e7;
  }

  .detail-value.message {
    font-weight: 500;
    color: #fafafa;
  }

  .detail-value.hash {
    font-family: monospace;
    font-size: 0.72rem;
    color: #60a5fa;
    word-break: break-all;
  }

  /* Diff */
  .diff-hashes {
    display: flex;
    align-items: center;
    justify-content: space-between;
    background: #27272a;
    padding: 6px 10px;
    border-radius: 6px;
    font-size: 0.75rem;
  }

  .hash-tag {
    display: flex;
    align-items: center;
    gap: 4px;
    color: #a1a1aa;
  }

  .hash-tag code {
    color: #60a5fa;
    font-weight: bold;
  }

  .diff-hint {
    font-size: 0.8rem;
    color: #eab308;
    margin: 4px 0;
  }

  .diff-output {
    margin: 0;
    padding: 8px;
    background: #111113;
    border-radius: 4px;
    border: 1px solid #2e2e36;
    font-size: 0.72rem;
    color: #d4d4d8;
    max-height: 220px;
    overflow-y: auto;
    white-space: pre-wrap;
    word-break: break-word;
  }

  .diff-empty {
    font-size: 0.8rem;
    color: #4ade80;
    margin: 0;
  }

  .diff-error {
    font-size: 0.8rem;
    color: #f87171;
    margin: 0;
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
