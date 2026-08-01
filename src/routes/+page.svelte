<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { Terminal } from "@xterm/xterm";
  import { onMount } from "svelte";
  import "@xterm/xterm/css/xterm.css";
  import { createGitgraph } from "@gitgraph/js";
  import { spawn } from "tauri-pty";

  let name = $state("");
  let greetMsg = $state("");
  let terminalElement: HTMLDivElement;
  let gitgraphElement: HTMLDivElement;

  let activeMenu = $state<string | null>(null);

  // fonction qu'on appelle lorsqu'on clique sur un bouton du menu

  function toggleMenu(command: string) {
    if (activeMenu === command) {
      // si la valeur passé à command est égale en valeur et en type à activeMenu
      activeMenu = null; // la valeur qu'on passe au menu active est null
    } else {
      activeMenu = command; // la valeur qu'on passe à la variable est command (le texte du bouton sur lequel on appuie)
    }
  }

  const dropdownGitActions = [
    {
      label: "Afficher branches",
      command: "git branch",
      subMenu: [
        { label: "branche local", command: "git branch" },
        { label: "branche distant", command: "git branch -r" },
        { label: "branche local + distant", command: "git branch -a" },
      ],
    },
    {
      label: "Changer de branche",
      command: "git checkout",
      subMenu: [
        { label: "branche local", command: "git branch" },
        { label: "branche distant", command: "git branch -r" },
      ],
    },
    {
      label: "Envoyer modification sur le repo distant",
      command: "git push",
    },
    {
      label: "git status",
      command: "git commit",
      subMenu: [
        { label: "Statut détaillé (par défaut)", command: "git status" },
        { label: "Statut compact", command: "git status -s" },
      ],
    },
    {
      label: "Envoyer les modifications",
      command: "git push",
      subMenu: [
        { label: "Push simple (branche actuelle)", command: "git push" },
        { label: "Publier une nouvelle branche (-u)", command: "git push -u" },
      ],
    },
  ];

  /* on définit une fonction qui lorsqu'on l'appelle donne qui inverse la valeur de la variable showBranchMenu 
     l'inverse de faux ==> vrai */

  onMount(() => {
    const term = new Terminal({ cursorBlink: true });
    term.open(terminalElement);
    const pty = spawn("bash", [], { cols: term.cols, rows: term.rows });
    term.onData((data) => pty.write(data));
    pty.onData((data) => term.write(data));

    // ---  code GitGraph ---
    const gitgraph = createGitgraph(gitgraphElement, {
      orientation: "horizontal",
      template: "metro",
    });

    // on créer un faux arbre git pour le moment
    const main = gitgraph.branch("main");
    main.commit("Initial commit");

    const feat = gitgraph.branch("feat/menu");
    feat.commit("Ajout du composant Dropdown");
    feat.commit("Ajout des sous-menus");

    main.merge(feat, "Merge branch 'feat/menu'");
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
              <button class="sub-item">
                {sub.label} ( {sub.command} )
              </button>
            {/each}
          </div>
        {/if}
      {/each}
    </div>

    <div class="preview-box" bind:this={gitgraphElement}></div>
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
    gap: 15px;
  }

  h1,
  p {
    color: white;
    font-weight: bold;
    font-family: "Inter", sans-serif;
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
    width: 30%;

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
    font-family: "Inter", sans-serif;
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
</style>
