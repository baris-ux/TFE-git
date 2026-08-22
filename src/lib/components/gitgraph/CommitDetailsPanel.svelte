<script lang="ts">
  import { CommitInteractionState } from "./CommitInteractionState.svelte";
  import CloseButton from "../ui/CloseButton.svelte";

  let { state }: { state: CommitInteractionState } = $props();
</script>

<div class="panel-card">
  <div class="panel-header">
    <div class="header-title">
      <span class="commit-badge">{state.selectedCommit?.id.slice(0, 7)}</span>
      <h3>Détails du commit</h3>
    </div>
    <CloseButton onclick={() => (state.commitInfoDisplayed = false)} />
  </div>

  <div class="details-list">
    <div class="detail-item">
      <span class="detail-label">Message</span>
      <p class="detail-value message">
        {state.selectedCommit?.message}
      </p>
    </div>
    <div class="detail-item">
      <span class="detail-label">Auteur</span>
      <p class="detail-value">
        {state.selectedCommit?.author}
      </p>
    </div>
    <div class="detail-item">
      <span class="detail-label">Hash complet</span>
      <code class="detail-value hash">{state.selectedCommit?.id}</code>
    </div>
    {#if state.selectedCommit?.parents?.length}
      <div class="detail-item">
        <span class="detail-label">Parent(s)</span>
        <code class="detail-value hash"
          >{state.selectedCommit.parents.join(", ")}</code
        >
      </div>
    {/if}
  </div>
</div>

<style>
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
</style>
