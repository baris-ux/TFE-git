<script lang="ts">
  import { commandDetails } from "$lib/config/commandDetails";

  let {
    selectedInfo,
    generateCommand,
  }: {
    selectedInfo: string;
    generateCommand: (Command: string) => void;
  } = $props();

  const info = $derived(commandDetails[selectedInfo]);
</script>

<!--{#if selectedInfo}
  {@const info = commandDetails[selectedInfo]}-->
<div class="info-box">
  {#if info}
    <h1>{info.title}</h1>
    <span class="badge {info.riskLevel}">{info.riskLevel}</span>
    <p>{info.description}</p>

    <code>{info.example}</code>
    <code>{info.output}</code>

    <button class="confirm-button" onclick={() => generateCommand(selectedInfo)}
      >confirmer</button
    >
  {:else}
    <h1>{selectedInfo}</h1>
    <p>
      La fiche de cours pour la commande {selectedInfo} viendra très prochainement
      !
    </p>
    <button class="confirm-button" onclick={() => generateCommand(selectedInfo)}
      >confirmer</button
    >
  {/if}
</div>

<style>
  .confirm-button {
    border: none;
    cursor: pointer;
    padding: 10px 5px;
    transition:
      background-color 150ms ease,
      box-shadow 150ms ease,
      transform 100ms ease;
  }

  .info-box {
    width: 260px;
    background-color: #2b2b2b;
    border: 1px solid #444444;
    border-radius: 8px;
    padding: 16px;
    color: #ffffff;
    display: flex;
    flex-direction: column;
    gap: 12px;
    height: fit-content;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);

    flex: 1;
  }

  .badge.safe {
    background-color: rgba(34, 197, 94, 0.15);
    color: #4ade80;
    border: 1px solid #22c55e;
  }

  .badge.normal {
    background-color: rgba(234, 179, 8, 0.15);
    color: #facc15;
    border: 1px solid #eab308;
  }

  .badge.danger {
    background-color: rgba(239, 68, 68, 0.15);
    color: #f87171;
    border: 1px solid #ef4444;
  }

  .badge {
    width: fit-content;
    padding: 5px 10px;
    border-radius: 5px;
    font-weight: bold;
  }
</style>
