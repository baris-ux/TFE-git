<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { Terminal } from "@xterm/xterm";
  import { onMount } from "svelte";
  import "@xterm/xterm/css/xterm.css";
  import { createGitgraph } from "@gitgraph/js";
  import { spawn } from "tauri-pty";
  import { open } from "@tauri-apps/plugin-dialog";
  import { path } from "@tauri-apps/api";

  let name = $state("");
  let greetMsg = $state("");
  let terminalElement: HTMLDivElement;
  let gitgraphElement: HTMLDivElement;
  let commits = $state<CommitInfo[]>([]);

  let activeMenu = $state<string | null>(null);
  let projectPath = $state<string | null>(null); // peut contenir un string (le path du dossier) ou alors null (si aucun dossier fournit)

  // fonction qu'on appelle lorsqu'on clique sur un bouton du menu

  function toggleMenu(command: string) {
    if (activeMenu === command) {
      // si la valeur passé à command est égale en valeur et en type à activeMenu
      activeMenu = null; // la valeur qu'on passe au menu active est null
    } else {
      activeMenu = command; // la valeur qu'on passe à la variable est command (le texte du bouton sur lequel on appuie)
    }
  }

  async function selectProject() {
    const selected = await open({
      directory: true, // permet de choisir un Folder
      multiple: false, // interdiction de choisir plusieur Folders
    });

    if (selected && typeof selected === "string") {
      projectPath = selected;
      loadGitHistory(selected);
    }
  }

  async function loadGitHistory(path: string) {
    console.log("Project selectionné : ", path);

    try {
      const isRepo = await invoke<boolean>("if_git_repository", {path}); // on récupère le résultat de la fonction rust sous forme de booleen

      if (isRepo){ // si isRepo est vrai
        commits = await invoke<CommitInfo[]>("get_git", { path }); // on appelle la structure rust et la fonction get_git en rust auquel on passe le paramètre le path du repo
        renderGitGraph(commits);
      } 
    }

    catch(error){
      console.error("Erreur lors de la vérification Git:", error);
    }
  }

  function renderGitGraph(commitsList: CommitInfo){

    gitgraphElement.innerHTML = "" // on vient vider le contenu avant de le déssiner pour éviter que plusieur abres git se superposent

    const gitgraph = createGitgraph(gitgraphElement, {
      orientation: "horizontal", 
      template: "metro",
    });

    const main = gitgraph.branch("main");

    for (const c of commits) {
      main.commit({
        hash: c.id.slice(0, 7), // hash raccourci (7 caractères), plus lisible
        subject: c.message,
        author: c.author,
      });
    }
  }


  /*  interfaceCommitInfo
      indique dans notre code svelte à quoi doit ressembler une un "objet" 
      commit avec ses clé et le type de valeur pour chaque clé 
      il ne contient pas d'information en tant que telle  
  */

  interface CommitInfo {
    id: string,
    message: string,
    author: string,
    parents: string[],
    branches: string[],
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
  });

</script>

<main class="container">
  <header class="header-bar">
    <div>
      <h1>Actions guidées</h1>
      <p>Projet actuel : <span class="project-path">{projectPath ?? "aucun projet selectionné"}</span></p>
    </div>

    <button class="open-btn" onclick={selectProject}>
      📂 Ouvrir un projet Git
    </button>
  </header>

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

  /* Styles pour le Header et le bouton "chosir projet"*/
  .header-bar {
    display: flex;
    justify-content: space-between;
    align-items: center;
    background-color: #1e1e1e;
    padding: 15px 20px;
    border-radius: 6px;
    border: 1px solid #333333;
  }

  .header-bar h1 {
    margin: 0 0 5px 0;
  }

  .header-bar p {
    margin: 0;
  }

  .project-path {
    color: #61afef;
    font-family: monospace;
  }

  .open-btn {
    background-color: #2c539e;
    color: white;
    border: none;
    padding: 10px 16px;
    font-size: 0.95rem;
    font-weight: bold;
    border-radius: 4px;
    cursor: pointer;
    transition: background-color 0.2s;
  }

  .open-btn:hover {
    background-color: #3b69c4;
  }
</style>
