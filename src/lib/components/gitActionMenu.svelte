<script lang="ts">
  import { dropdownGitActions } from "$lib/config/GitActionsMenu";
  import { commandDetails } from "$lib/config/commandDetails" 

  let { 
    activeView, 
    activeMenu, 
    toggleMenu, 
    generateCommand 
  } = $props(); 

  // $props() déja expliqué dans Header.svelte

  let selectedInfo = $state<string | null>(null);

  function toggleInfo(command : string){
    if (selectedInfo === command){ // par exemple si  "git status" === "git status" ca veut dire qu'on a ouvert le bouton, 
      selectedInfo = null;  // dans ce cas la variable passe à null pour le refermer 
    }
    else{ // par exemple si on a null === "git status"
      selectedInfo = command; 
    }
  }

</script>

<div 
  class="dropdown-content" // dropdown-content est le conteneur qui va contenir, notre liste d'action guidé, les "sous actions" ainsi que la fiche d'information
  class:hidden={activeView === "tree"} 
  class:full-width={activeView === "actions"}
>
  <div class = "menu-list"> 
      {#each dropdownGitActions as action}
      <!-- on vient créer tous les boutons dans notre liste d'objets -->
      <button
        class="dropdown-item"
        onclick={() => toggleMenu(action.command)}
      >
        {action.label}
      </button>

      {#if activeMenu === action.command && action.subMenu}
        <div class="sub-menu">
          {#each action.subMenu as sub}

          <div class="sub-item-row">
            <button 
              class="sub-item" 
              onclick={() => generateCommand(sub.command)}
            >
              {sub.label} ( {sub.command} )
            </button>

            <button 
              class = "fiche-information"
              onclick={() => toggleInfo(sub.command)}
            >
              i
            </button>
          </div>

          {/each}
        </div>
      {/if}
    {/each}
  </div>
  
  {#if selectedInfo}
      {@const info = commandDetails[selectedInfo]}

      <div class="info-box">
        {#if info}
          <h1>{info.title}</h1>
          <p>{info.description}</p>

          {#if info.example}
            <code>{info.example}</code>
          {/if}
        {:else}
          <h1>{selectedInfo}</h1>
          <p>La fiche de cours pour la commande {selectedInfo} viendra très prochainement !</p>
        {/if}
      </div>
    {/if}

</div>

<style>

  .dropdown-content {
    background-color: #505050;
    flex : 1;

    display: flex;
    flex-direction: row; /* pour que les deux class enfant soient aligné à l'horizontal" */
    padding: 10px;

    overflow-y: auto;
    box-sizing: border-box;
    border-radius: 6px;
    gap : 15px;
    opacity: 1;
    transition: flex 0.4s cubic-bezier(0.4, 0, 0.2, 1), opacity 0.25s ease-in-out, padding 0.4s ease;
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

  .fiche-information {
    cursor : pointer;
    border : none;
    background-color: #3a3a3a;
    font-weight: bold;
    color: #ffffff;
    padding: 10px 14px;
    border-radius: 4px;
    transition: background-color 0.2s, color 0.2s;
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

      flex : 1;
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