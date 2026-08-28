<script lang="ts">
  import { dropdownGitActions } from "$lib/config/GitActionsMenu";
  import LegendInfoPanel from "./LegendInfoPanel.svelte";
  import Switch from "../ui/Switch.svelte";
  import CommandInfoPanel from "./CommandInfoPanel.svelte";
  import { invoke } from "@tauri-apps/api/core";

  let { activeView, activeMenu, toggleMenu, generateCommand, path } = $props();
  let sliderValue = $state(true);
  let selectedInfo = $state<string | null>(null);
  let legendClicked = $state(false);
  let gitState = $state(false);
  let result = $state<string>("");

  let workingArea = $derived(
    result
      .split("\n")
      .filter((line) => line.length > 0)
      .filter(
        (item) =>
          (item[0] === "?" && item[1] === "?") ||
          item[1] === "M" ||
          item[1] === "D",
      ),
  );

  let staggingArea = $derived(
    result
      .split("\n")
      .filter((line) => line.length > 0)
      .filter((item) => item[0] === "A" || item[0] === "M"),
  );

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

  async function getActifFiles() {
    result = await invoke<string>("get_actif_files", { path });
  }

  function toggleGitState() {
    if (gitState) {
      gitState = false;
      return;
    }

    gitState = true;
    selectedInfo = null;
    legendClicked = false;
    getActifFiles();
  }
</script>

<div
  class="dropdown-content"
  class:hidden={activeView === "tree"}
  class:full-width={activeView === "actions"}
>
  <div class="menu-list">
    <header class="git-actions-header">
      <div class="title-group">
        <h1>Menu d'actions guidées</h1>
        <button class="legend-btn" onclick={toggleLegendModal}>
          Légende des risques
        </button>

        {#if path !== null}
          <button class="legend-btn" onclick={toggleGitState}>État Git</button>
        {/if}
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
    {#each dropdownGitActions as action (action.command)}
      <!-- on vient créer tous les boutons dans notre liste d'objets -->
      <button class="dropdown-item" onclick={() => toggleMenu(action.command)}>
        {action.label}
      </button>

      {#if activeMenu === action.command && action.subMenu}
        <div class="sub-menu">
          {#each action.subMenu as sub (sub.command)}
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

  {#if selectedInfo}
    <CommandInfoPanel
      {selectedInfo}
      {generateCommand}
      oncloseinfopanel={() => {
        selectedInfo = null;
      }}
    />
  {:else if legendClicked}
    <LegendInfoPanel onclose={() => (legendClicked = false)} />
  {:else if gitState}
    {#if result === ""}
      <p>aucun changement dans les 3 areas</p>
    {:else}
      <div class="git-columns">
        <div class="column">
          <h3>Working Area ({workingArea.length})</h3>
          {#each workingArea as item}
            <p>{item}</p>
          {/each}
        </div>

        <div class="column">
          <h3>Stagging Area</h3>
          {#each staggingArea as item}
            <p>{item}</p>
          {/each}
        </div>

        <div class="column">
          <h3>Repository</h3>
          <p class="repo-info">Dernier commit validé (HEAD)</p>
        </div>
      </div>
    {/if}
  {/if}
</div>

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

  .hidden {
    flex: 0;
    width: 0;
    padding: 0;
    opacity: 0;
    pointer-events: none;
    border: none;
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

  .git-columns {
    display: flex;
    flex-direction: row;
    gap: 15px;
  }

  .column {
    flex: 1;
    background-color: #2b2b2b;
    border-radius: 6px;
    padding: 12px;
    gap: 8px;
    border-top: 3px solid #666;
  }

  .column:nth-child(1) {
    border-top-color: #f87171;
  }

  .column:nth-child(2) {
    border-top-color: #4ade80;
  }

  .column:nth-child(3) {
    border-top-color: #60a5fa;
  }

  .column p {
    margin: 0;
    padding: 6px 8px;
    background-color: #1f1f1f;
    border-radius: 4px;
    font-family: monospace;
    font-size: 0.8rem;
    color: #e0e0e0;
    word-break: break-all;
  }
</style>
