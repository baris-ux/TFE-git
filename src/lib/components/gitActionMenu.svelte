<script lang="ts">
  import { dropdownGitActions } from "$lib/config/GitActionsMenu";

  let { 
    activeView, 
    activeMenu, 
    toggleMenu, 
    generateCommand 
  } = $props(); 

  // $props() permet de récupérer les données dynamique du composant parent, ici en l'occurence +page.svelte dans lequel
  // sont définit ces variable et ces fonction, on les récupère pour que notre code puisse fonctionner correctement

</script>

<div 
  class="dropdown-content"
  class:hidden={activeView === "tree"}
  class:full-width={activeView === "actions"}
>
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
          <button 
            class="sub-item" 
            onclick={() => generateCommand(sub.command)}
          >
            {sub.label} ( {sub.command} )
          </button>
        {/each}
      </div>
    {/if}
  {/each}
</div>

<style>

  .dropdown-content {
    background-color: #505050;
    flex : 1;

    display: flex;
    flex-direction: column;
    gap: 5px;
    padding: 10px;

    overflow-y: auto;
    box-sizing: border-box;
    border-radius: 6px;
  }

  .dropdown-item {
    width: 70%;
    padding: 10px;
    cursor: pointer;
    border: none;
    background-color: #666666;
    color: white;
    text-align: left;
    border-radius: 4px;
    transition: background-color 0.2s;
  }

  .dropdown-item:hover {
    background-color: #888888;
  }

  .sub-menu {
    width: 70%;
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

  .dropdown-content {
    flex: 1;
    opacity: 1;
    overflow: auto;
    box-sizing: border-box;
    border-radius: 6px;

    /* 👉 AJOUTER CETTE LIGNE (Anime le flex, l'opacité et le padding) */
    transition: flex 0.4s cubic-bezier(0.4, 0, 0.2, 1), opacity 0.25s ease-in-out, padding 0.4s ease;
  }


</style>