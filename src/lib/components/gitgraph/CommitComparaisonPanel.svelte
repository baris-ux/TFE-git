<script lang="ts">
  import CloseButton from "../ui/CloseButton.svelte";
  import { CommitInteractionState } from "./CommitInteractionState.svelte";
  let { state }: { state: CommitInteractionState } = $props();
</script>

<div class="panel-card diff-card">
  <div class="panel-header">
    <div class="header-title">
      <h3>Comparaison</h3>
    </div>
    <CloseButton onclick={() => state.cancelComparaison()} />
  </div>

  <div class="diff-hashes">
    <div class="hash-tag">
      <span>Base :</span>
      <code>{state.firstHash?.slice(0, 7)}</code>
    </div>
    <span class="arrow">➔</span>
    <div class="hash-tag">
      <span>Cible :</span>
      <code>{state.secondHash ? state.secondHash.slice(0, 7) : "..."}</code>
    </div>
  </div>

  {#if state.secondHash === null}
    <p class="diff-hint">Cliquez sur un second commit dans le graphe...</p>
  {:else if state.diffResult}
    <pre class="diff-output">{state.diffResult}</pre>
    <!-- si diffResult est une chaine vide (j'ai beaucoup galéré sur celui la ...) -->
  {:else if state.diffResult === ""}
    <p class="diff-empty">Aucune différence détectée entre ces commits.</p>
  {:else if state.diffError}
    <p class="diff-error">{state.diffError}</p>
  {/if}
</div>

<style>
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

  .diff-card {
    width: 340px;
  }
</style>
