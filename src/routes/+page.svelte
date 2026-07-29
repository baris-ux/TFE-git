<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { Terminal } from '@xterm/xterm';
  import { onMount } from "svelte";
  import "@xterm/xterm/css/xterm.css";

  let name = $state("");
  let greetMsg = $state("");
  let terminalElement: HTMLDivElement;

  let showBranchMenu = $state(false); // par défaut on n'a pas appuyer sur le bouton donc l'état est sur false

  function toggleBranchMenu() {
    showBranchMenu = !showBranchMenu;
  }

  /* on définit une fonction qui lorsqu'on l'appelle donne qui inverse la valeur de la variable showBranchMenu 
     l'inverse de faux ==> vrai */

  onMount(() => {
    const term = new Terminal({
      cursorBlink: true
    });

    term.open(terminalElement);
    term.write("Hello from xterm!");

    term.onData((data) => {
      // Pour l'instant, on se contente de ré-afficher ce que l'utilisateur tape
      term.write(data);
    });
  });

  async function greet(event: Event) {
    event.preventDefault();
    greetMsg = await invoke("greet", { name });
  }

</script>

<main class="container">

  <h1>Actions guidées</h1>

  <p>Cliquer sur une action pour générer la commande git</p>

  <div class="content-layout">
    
    <div class="dropdown-content">

      <div class = "menu-group">

        <button class="dropdown-item" onclick={toggleBranchMenu}>
          Afficher branches<br>(git branch)
        </button>

        {#if showBranchMenu} 
          <div class="sub-menu">
            <button class="sub-item">
              branche local (git branch)
            </button>
            <button class="sub-item">
              branche distant (git branch -r)
            </button>
            <button class="sub-item">
              branche local + distant (git branch - a) 
            </button>
          </div>
        {/if}
      </div>

      <button class="dropdown-item">Changer de branche<br>(git checkout)</button>
      <button class="dropdown-item">envoyer modification sur le repo distant<br>(git push)</button>
    </div>

    <div class="preview-box">
      <span>À venir</span>
    </div>
  </div>

  <div bind:this={terminalElement} class="terminal-container"></div>

</main>

<style>
  :global(body) {
    background-color: black;
    margin: 0;
    padding: 0;
  }

  .container {
    display: flex;
    flex-direction: column;
    height: 100vh;
    padding: 20px;
    box-sizing: border-box;
    gap : 15px;
  }

  h1, p {
    color: white;
    font-weight: bold;
    font-family: 'Inter', sans-serif;
    margin-top: 0;
    margin-bottom: 15px;
  }

  .content-layout {
    display: flex;
    flex-direction: row;
    gap: 15px;
    width: 100%;
    flex: 4; 
    min-height: 0; 
  }

  .dropdown-content {
    background-color: #505050;
    width : 30%;

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


  /* Box "À venir" */
  .preview-box {
    background-color: #2a2a2a;
    border: 2px dashed #666666;
    flex: 1; /* Prend l'autre 50% de l'espace */
    display: flex;
    justify-content: center;
    align-items: center;
    color: #aaaaaa;
    font-family: 'Inter', sans-serif;
    font-size: 1.2rem;
    font-weight: bold;
    border-radius: 6px;
    box-sizing: border-box;
  }

  .terminal-container {
    flex: 1;
    width: 100%;
    background-color: #000;
    border-radius: 6px;
    overflow: hidden;
    border: 1px dashed white;
  }

  .sub-menu{
    width : 70%;
    display: flex;
    flex-direction : column;
    gap : 5px;
  }

  .sub-item{
    border : none;
    cursor : pointer;
    background-color: #444444;
    color: #dddddd;
    padding : 10px;
    text-align : left;
    box-sizing: border-box; /* à spécifier sinon c'est content-box par défaut */
  }

  .sub-item:hover {
    background-color: #555555;
    color: white;
  }

</style>