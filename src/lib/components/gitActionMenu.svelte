<script lang="ts">
  import { dropdownGitActions } from "$lib/config/GitActionsMenu";
  import { commandDetails } from "$lib/config/commandDetails";
  import CloseButton from "./CloseButton.svelte";
  import Switch from "./Switch.svelte";

  let { activeView, activeMenu, toggleMenu, generateCommand } = $props();
  let sliderValue = $state(true);
  let selectedInfo = $state<string | null>(null);
  let legendClicked = $state(false);

  const riskLegends = {
    safe: "Le niveau 'safe' désigne les commandes en lecture seule : leur exécution est sans aucun danger pour vos fichiers ou votre historique.",
    normal:
      "Le niveau 'normal' désigne les commandes qui créent, modifient ou suppriment des éléments en local, mais de manière réversible.",
    danger:
      "Le niveau 'danger' désigne les commandes qui effectuent des modifications irréversibles ou qui risquent d'écraser du travail non sauvegardé.",
  };

  function toggleInfo(command: string) {
    if (selectedInfo === command) {
      // par exemple si  "git status" === "git status" ca veut dire qu'on a ouvert le bouton,
      selectedInfo = null; // dans ce cas la variable passe à null pour le refermer
    } else {
      // par exemple si on a null === "git status"
      selectedInfo = command;
    }
  }

  function toggleLegendModal() {
    legendClicked = true;
  }
</script>

<div
  class="dropdown-content"
  // dropdown-content est le conteneur qui va contenir, notre liste d'action guidé, les "sous actions" ainsi que la fiche d'information
  class:hidden={activeView === "tree"}
  class:full-width={activeView === "actions"}
>
  <div class="menu-list">
    <header class="git-actions-header">
      <div class="title-group">
        <h1>Menu d'actions guidées</h1>
        <button class="legend-btn" role="button" onclick={toggleLegendModal}>
          Légende des risques
        </button>
      </div>

      <div class="toggle-group">
        <Switch
          bind:value={sliderValue}
          label="Mode explications"
          fontSize={18}
          design="slider"
        />
        <span class="status-indicator" class:active={sliderValue}>
          {sliderValue ? "Actif" : "Désactivé"}
        </span>
      </div>
    </header>
    {#each dropdownGitActions as action}
      <!-- on vient créer tous les boutons dans notre liste d'objets -->
      <button class="dropdown-item" onclick={() => toggleMenu(action.command)}>
        {action.label}
      </button>

      {#if activeMenu === action.command && action.subMenu}
        <div class="sub-menu">
          {#each action.subMenu as sub}
            <div class="sub-item-row">
              <button
                class="sub-item"
                onclick={() =>
                  sliderValue
                    ? toggleInfo(sub.command)
                    : generateCommand(sub.command)}
              >
                {sub.label} ( {sub.command} )
              </button>
            </div>
          {/each}
        </div>
      {/if}
    {/each}
  </div>

  {#if legendClicked}
    <div class="legend-box">
      <CloseButton onclick={() => (legendClicked = false)} />
      <h3>Légende des niveaux de risques</h3>
      <p>
        Lorsque le mode explications est actif, un badge de couleur indique le
        niveau d'impact de l'action sélectionnée. Voici la signification de
        chacun de ces badges :
      </p>
      <ul class="legend-list">
        {#each Object.entries(riskLegends) as [key, text]}
          <li>
            <div class="badge-wrapper">
              <span class="badge {key}">{key}</span>
            </div>
            <p class="risk-legend">{text}</p>
          </li>
        {/each}
      </ul>
    </div>
  {:else if selectedInfo}
    {@const info = commandDetails[selectedInfo]}

    <div class="info-box">
      {#if info}
        <h1>{info.title}</h1>
        <span class="badge {info.riskLevel}">{info.riskLevel}</span>
        <p>{info.description}</p>

        {#if info.example}
          <code>{info.example}</code>
        {/if}

        <button
          class="confirm-button"
          onclick={() => generateCommand(info.example)}>confirmer</button
        >
      {:else}
        <h1>{selectedInfo}</h1>
        <p>
          La fiche de cours pour la commande {selectedInfo} viendra très prochainement
          !
        </p>
        <button>confirmer</button>
      {/if}
    </div>
  {/if}
</div>

<!-- HTML 
<button class="button-86" role="button">Button 86</button>

/* CSS 
.button-86 {

}

!-->

<style>
  .dropdown-content {
    background-color: #505050;
    flex: 1;

    display: flex;
    flex-direction: row; /* pour que les deux class enfant soient aligné à l'horizontal" */
    padding: 10px;

    overflow-y: auto;
    box-sizing: border-box;
    border-radius: 6px;
    gap: 15px;
    opacity: 1;
    transition:
      flex 0.4s cubic-bezier(0.4, 0, 0.2, 1),
      opacity 0.25s ease-in-out,
      padding 0.4s ease;
  }

  .dropdown-item {
    padding: 10px;
    cursor: pointer;
    border: none;
    background-color: #666666;
    color: white;
    text-align: left;
    border-radius: 4px;
    transition: background-color 0.2s;
  }

  .menu-list {
    display: flex;
    flex-direction: column;
    gap: 10px;
    flex: 1;
  }

  .dropdown-item:hover {
    background-color: #888888;
  }

  .sub-menu {
    display: flex;
    flex-direction: column;
    gap: 5px;
  }

  .sub-item {
    border: none;
    cursor: pointer;
    background-color: #444444;
    color: #dddddd;
    padding: 10px;
    text-align: left;
    box-sizing: border-box; /* à spécifier sinon c'est content-box par défaut */
  }

  .sub-item:hover {
    background-color: #555555;
    color: white;
  }

  .confirm-button {
    border: none;
    cursor: pointer;
    padding: 10px 5px;
    transition:
      background-color 150ms ease,
      box-shadow 150ms ease,
      transform 100ms ease;
  }

  .fiche-information {
    cursor: pointer;
    border: none;
    background-color: #3a3a3a;
    font-weight: bold;
    color: #ffffff;
    padding: 10px 14px;
    border-radius: 4px;
    transition:
      background-color 0.2s,
      color 0.2s;
  }

  .fiche-information:hover {
    background-color: #64b5f6;
    color: #1e1e1e;
  }

  .sub-item-row {
    display: flex;
    align-items: center;
    gap: 6px;
    width: 100%;
  }

  .sub-item {
    flex: 1;
    min-width: 0;
  }

  .legend-box,
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

  .hidden {
    flex: 0;
    width: 0;
    padding: 0;
    opacity: 0;
    pointer-events: none;
    border: none;
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

  /* ----------------------- le css du header ----------------------- */

  .git-actions-header {
    display: flex;
    flex-direction: column;
    border-bottom: 1px solid #444444;
    gap: 20px;
  }

  .title-group {
    display: flex;
    align-items: center;
    justify-content: flex-start;
    gap: 25px;
  }

  .title-group h1 {
    margin: 0;
    color: #ffffff;
    font-weight: bold;
  }
  .toggle-group {
    display: flex;
    align-items: center;
    gap: 10px;
  }

  .status-indicator {
    color: #888888;
  }

  .status-indicator.active {
    color: #4ade80; /* Vert discret */
  }

  /* css du bouton repris sur un site et non généré par IA */

  .legend-btn {
    all: unset;
    height: 30px;
    font-size: 16px;
    background: transparent;
    border: none;
    position: relative;
    color: #f0f0f0;
    cursor: pointer;
    z-index: 1;
    padding: 10px 20px;
    display: flex;
    align-items: center;
    justify-content: center;
    white-space: nowrap;
    user-select: none;
    -webkit-user-select: none;
    touch-action: manipulation;
  }

  .legend-btn::after,
  .legend-btn::before {
    content: "";
    position: absolute;
    bottom: 0;
    right: 0;
    z-index: -99999;
    transition: all 0.4s;
  }

  .legend-btn::before {
    transform: translate(0%, 0%);
    width: 100%;
    height: 100%;
    background: #28282d;
    border-radius: 10px;
  }

  .legend-btn::after {
    transform: translate(10px, 10px);
    width: 35px;
    height: 35px;
    background: #ffffff15;
    backdrop-filter: blur(5px);
    -webkit-backdrop-filter: blur(5px);
    border-radius: 50px;
  }

  .legend-btn:hover::before {
    transform: translate(5%, 20%);
    width: 110%;
    height: 110%;
  }

  .legend-btn:hover::after {
    border-radius: 10px;
    transform: translate(0, 0);
    width: 100%;
    height: 100%;
  }

  .legend-btn:active::after {
    transition: 0s;
    transform: translate(0, 5%);
  }
</style>
