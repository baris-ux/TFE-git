<script lang="ts">
  import type { Snippet } from "svelte";

  let viewportElement = $state<HTMLDivElement>();

  let panX = $state(0); // c'est le décalage en x entre le <div> de canvas-viewport et le <div> canvas-world
  let panY = $state(0); // pareil mais en y
  let zoom = $state(1);
  let isDragging = $state(false);
  let dragStartX = $state(0);
  let dragStartY = $state(0);

  let {
    activeView,
    graph,
    children,
  }: {
    activeView: "split" | "actions" | "tree";
    graph?: Snippet;
    children?: Snippet;
  } = $props();

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
    {#if graph}
      {@render graph()}
    {/if}
  </div>
  <!--
    {#key commits}
      "Utilise la variable commits comme clé d'identification de tout ce qui se trouve entre {#key} et {/key}.
      Tant que cette clé ne change pas, laisse le <div> tranquille. 
      Mais dès que la valeur ou la référence de commits change, considère que ce bloc est périmé : 
      détruis-le entièrement et reconstruis-le à neuf." 
      
      --gemini j'ai trouvé que son explication était bien meilleur que le miens, 

      ici il permet de redéssiner le gitgraph, il permet d'éviter le warning obtenu avec le github action lorsqu'on utilisait ==> gitgraphElement.textContent = ""
      <div bind:this={gitgraphElement}></div>
    {/key}
  -->

  <div class="zoom-controls">
    <button onclick={() => (zoom = Math.min(zoom * 1.15, 3.0))}>+</button>
    <span>{Math.round(zoom * 100)}%</span>
    <button onclick={() => (zoom = Math.max(zoom / 1.15, 0.3))}>−</button>
    <button onclick={resetView} title="Recentrer">⟲</button>
  </div>
</div>
{#if children}
  {@render children()}
{/if}

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

  .hidden {
    flex: 0;
    width: 0;
    padding: 0;
    opacity: 0;
    pointer-events: none;
    border: none;
  }
</style>
