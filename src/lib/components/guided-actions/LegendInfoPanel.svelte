<script lang="ts">
  import CloseButton from "../ui/CloseButton.svelte";
  let { onclose } = $props<{ onclose: () => void }>();

  const riskLegends = {
    safe: "Le niveau 'safe' désigne les commandes en lecture seule : leur exécution est sans aucun danger pour vos fichiers ou votre historique.",
    normal:
      "Le niveau 'normal' désigne les commandes qui créent, modifient ou suppriment des éléments en local, mais de manière réversible.",
    danger:
      "Le niveau 'danger' désigne les commandes qui effectuent des modifications irréversibles ou qui risquent d'écraser du travail non sauvegardé.",
  };
</script>

<div class="legend-box">
  <CloseButton onclick={onclose} />
  <h3>Légende des niveaux de risques</h3>
  <p>
    Lorsque le mode explications est actif, un badge de couleur indique le
    niveau d'impact de l'action sélectionnée. Voici la signification de chacun
    de ces badges :
  </p>
  <ul class="legend-list">
    {#each Object.entries(riskLegends) as [key, text] (key)}
      <li>
        <div class="badge-wrapper">
          <span class="badge {key}">{key}</span>
        </div>
        <p class="risk-legend">{text}</p>
      </li>
    {/each}
  </ul>
</div>

<style>
  .legend-box {
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

  .badge {
    width: fit-content;
    padding: 5px 10px;
    border-radius: 5px;
    font-weight: bold;
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
</style>
