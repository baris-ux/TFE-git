<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  import { open } from "@tauri-apps/plugin-dialog";

  import Header from "$lib/components/Header.svelte";
  import GitActionMenu from "$lib/components/gitActionMenu.svelte";
  import GitGraph from "$lib/components/gitGraph.svelte";
  import Terminal from "$lib/components/terminal.svelte";

  import { type CommitInfo } from "$lib/config/GitActionsMenu";

  //on vient stocker la référence vers un div contenu dans la DOM, on ne stocke pas le div en elle même !
  // comme on utilise des package pour le terminal et pour dessiner le git tree on vient indiquer ou les déssiner
  // cela permet d'éviter document.getElementById() car nos composant svelte son réutilisable

  let commits = $state<CommitInfo[]>([]);

  let activeMenu = $state<string | null>(null);
  let projectPath = $state<string | null>(null); // peut contenir un string (le path du dossier) ou alors null (si aucun dossier fournit) ==> la valeur par défaut est null

  let activeView = $state<"split" | "actions" | "tree">("split"); // cette variable va permettre de stocker la vue, écrant scinder, full écran sur le menu d'action ou alors full écran sur le graphe visuelle
  let terminalRef = $state<ReturnType<typeof Terminal>>();

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
      multiple: false, // interdiction de choisir plusieurs Folders
    });

    if (selected && typeof selected === "string") {
      projectPath = selected;
      loadGitHistory(selected);
      // à noter que le path passé en paramètre corresspond à un vrai dossier sur notre ordinateur mais il peut s'agir
      // d'un projet git ou non !
    }
  }

  async function loadGitHistory(path: string) {
    // cette fonction permet uniquement de vérfier si le dossier sélectionné est un projet git

    try {
      const isRepo = await invoke<boolean>("if_git_repository", { path }); // on récupère le résultat de la fonction rust sous forme de booleen

      if (isRepo) {
        // si isRepo est vrai
        commits = await invoke<CommitInfo[]>("get_git", { path }); // on appelle la structure rust et la fonction get_git en rust auquel on passe le paramètre le path du repo
        //renderGitGraph(commits);
      } else {
        commits = [];
        //gitgraphElement.innerText = "Ce n'est pas un projet git ..."
      }
    } catch (error) {
      console.error("Erreur lors de la vérification Git:", error);
    }
  }

  function generateCommand(command: string) {
    if (terminalRef) {
      // si le pty n'exsite pas
      terminalRef.sendCommand(`${command}`);
    }
  }

  function toggleView(targetView: "actions" | "tree") {
    if (activeView == targetView) {
      // si la vue active est actions ou tree (donc c'est la vue scindé) on vient scindé l'écran
      activeView = "split";
    } else {
      // si la vue active n'est pas actions ou tree (c'est qu'il est scindé en deux) alors la vue active sera celle selectionné
      activeView = targetView;
    }
  }
</script>

<main class="container">
  <Header {projectPath} {toggleView} {selectProject} />

  <div class="content-layout">
    <GitActionMenu {activeView} {activeMenu} {toggleMenu} {generateCommand} />

    <GitGraph {activeView} {commits} />
  </div>

  <Terminal />
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

  .content-layout {
    display: flex;
    flex-direction: row;
    gap: 15px;
    width: 100%;
    flex: 4;
    min-height: 0;
  }
</style>
