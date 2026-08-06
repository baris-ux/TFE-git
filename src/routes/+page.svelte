<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { Terminal } from "@xterm/xterm";
  import { onMount } from "svelte";
  import "@xterm/xterm/css/xterm.css";
  import { createGitgraph } from "@gitgraph/js";
  import { spawn } from "tauri-pty";
  import { open } from "@tauri-apps/plugin-dialog";
  import { path } from "@tauri-apps/api";

  import Header from "$lib/components/Header.svelte";
  import GitActionMenu from "$lib/components/gitActionMenu.svelte";

  import { dropdownGitActions, type CommitInfo } from "$lib/config/GitActionsMenu";
  import { myGitTheme } from "$lib/config/gitTheme";

  let terminalElement: HTMLDivElement;
  let gitgraphElement: HTMLDivElement;

  //on vient stocker la référence vers un div contenu dans la DOM, on ne stocke pas le div en elle même !
  // comme on utilise des package pour le terminal et pour dessiner le git tree on vient indiquer ou les déssiner
  // cela permet d'éviter document.getElementById() car nos composant svelte son réutilisable

  let commits = $state<CommitInfo[]>([]);

  let activeMenu = $state<string | null>(null); 
  let projectPath = $state<string | null>(null); // peut contenir un string (le path du dossier) ou alors null (si aucun dossier fournit)

  let pty: any = $state(null);

  let isDragging = false // détection du clique gauche
  let startX = 0;
  let startY = 0;
  let scrollLeft = 0;
  let scrollTop = 0;

  let activeView = $state<"split" | "actions" | "tree">("split"); // cette variable va permettre de stocker la vue, écrant scinder, full écran sur le menu d'action ou alors full écran sur le graphe visuelle


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

  async function loadGitHistory(path: string) { // cette fonction permet uniquement de vérfier si le dossier sélectionné est un projet git
    console.log("Project selectionné : ", path);

    try {
      const isRepo = await invoke<boolean>("if_git_repository", {path}); // on récupère le résultat de la fonction rust sous forme de booleen

      if (isRepo){ // si isRepo est vrai
        commits = await invoke<CommitInfo[]>("get_git", { path }); // on appelle la structure rust et la fonction get_git en rust auquel on passe le paramètre le path du repo
        renderGitGraph(commits);
      }
      else{
        commits = [];
        gitgraphElement.innerText = "Ce n'est pas un projet git ..."
      }
    }

    catch(error){
      console.error("Erreur lors de la vérification Git:", error);
    }
  }

export function renderGitGraph(commits: any[]) {
  
  if (!gitgraphElement) { // si la référence de div est vide (null / undefined), renvoie true  
    return; // le return permet d'empêcher le reste de la fonction de s'executer si c'est vrai 
  } 


  gitgraphElement.innerHTML = ""; // On néttoie la DOM à chaque fois quela fonction renderGitGraph est appelé pour éviter pour déssiner un nouvel arbre git


  const gitgraph = createGitgraph(gitgraphElement, { template: myGitTheme });

  const branches: Record<string, any> = {}; // on créer un objet vide, Record<> indique qu'il s'agit d'un objet, string indique que la clé sera du string, any indique que la valeur peut etre de n'importe quel type
  branches["main"] = gitgraph.branch("main"); // Reçoit le bleu par défaut

  const reversedCommits = [...commits].reverse(); // reverse inverse l'ordre des éléments une liste, on le reverse pour déssiner du bas (commit les plus ancien) vers le haut (commit les plus récents)
  const totalCommits = reversedCommits.length

  reversedCommits.forEach((c, index) => {
    const branchName = c.branch || "main";

    if (!branches[branchName]) {
      branches[branchName] = branches["main"].branch(branchName);
    }

    const isHead = index === totalCommits - 1;

    branches[branchName].commit({
      subject: c.message,
      hash: c.hash,
      author: c.author,
      tag: isHead ? "HEAD" : undefined
    });
  });
}

  onMount(() => {                                         // onMount est une fonction Svelte qui s'execute une seule fois lors de l'initialisation de la page, 
    const term = new Terminal({ cursorBlink: true });     // on vient créer un visuel de terminal, c'est une coquille vide ou l'on peut rien y faire à part écrire, on a seulelement le clignotement du cureur
    term.open(terminalElement);                           // on vient injecter le code du package xterm dans terminalElement, (qui pour rappel vient contenir une référence div dans la dom)
    
    pty = spawn("bash", [], { cols: term.cols, rows: term.rows }); // on vient générer le programme bash de notre OS, [] spécifie les options au démaragge du bash ici rien pour un démarage du bash par défaut 
    
    term.onData((data) => pty.write(data));               // quand on vient taper des caractère elles sont dorénavent transmit au pty. il s'active même une fois que la fonction onMount est finei
    pty.onData((data: string) => {                        // onData permet d'écouter l'arrivé de donnée, à l'arrivé on execute une fonction
      term.write(data);                                   // on écrit dans le composant xterm le résutlat renvoyé par le bash
    })



    if (gitgraphElement) {
          // 1. Clic enfoncé : Mémorise le point de départ (SANS 'const' devant startX/startY)
          gitgraphElement.addEventListener("mousedown", (event) => {
            isDragging = true;
            startX = event.pageX - gitgraphElement.offsetLeft; 
            startY = event.pageY - gitgraphElement.offsetTop;
            scrollLeft = gitgraphElement.scrollLeft;
            scrollTop = gitgraphElement.scrollTop;
          });

          // 2. Relâchement / Sortie de zone
          const stopDragging = () => { isDragging = false; };
          gitgraphElement.addEventListener("mouseup", stopDragging);
          gitgraphElement.addEventListener("mouseleave", stopDragging);

          // 3. Glissement : Effectue le calcul de déplacement
          gitgraphElement.addEventListener("mousemove", (e) => {
            if (!isDragging) return;
            e.preventDefault();
            const x = e.pageX - gitgraphElement.offsetLeft;
            const y = e.pageY - gitgraphElement.offsetTop;
            gitgraphElement.scrollLeft = scrollLeft - (x - startX);
            gitgraphElement.scrollTop = scrollTop - (y - startY);
          });
        }
    }
  );


  function generateCommand(command: string){
    if (pty){ // si le pty n'exsite pas 
      pty.write(`${command}`);
    }
  }


  function toggleView(targetView: "actions" | "tree"){
    if (activeView == targetView){ // si la vue active est actions ou tree (donc c'est la vue scindé) on vient scindé l'écran
      activeView = "split";
    }
    else { // si la vue active n'est pas actions ou tree (c'est qu'il est scindé en deux) alors la vue active sera celle selectionné 
      activeView = targetView;
    }
  }

</script>


<main class="container">

  <Header {projectPath} {toggleView} {selectProject} />

  <div class="content-layout">

    <GitActionMenu 
      {activeView} 
      {activeMenu} 
      {toggleMenu} 
      {generateCommand} 
    />

    <div 
      class="tree-wrapper"
      class:hidden={activeView === "actions"}  
      class:full-width={activeView === "tree"} 
      bind:this={gitgraphElement} 
    ></div>
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

  .content-layout {
    display: flex;
    flex-direction: row;
    gap: 15px;
    width: 100%;
    flex: 4;
    min-height: 0;
  }

  .tree-wrapper {

    background-color: #2a2a2a;
    border: 2px dashed #666666;
    flex: 1;
    color: #aaaaaa;
    font-family: "Inter", sans-serif;
    font-size: 1.2rem;
    font-weight: bold;
    border-radius: 6px;
    box-sizing: border-box;

    display: grid;
    place-items: center;
    overflow: auto;         /* auto permet à ce que la barre de scroll apparait si l'element dépasse */
  }

  .tree-wrapper :global(svg) {
    padding: 60px; 
  }

  .tree-wrapper :global(svg circle) { 
    cursor: pointer;
  }

  .tree-wrapper:active {
    cursor: grabbing;
  }


  .terminal-container {
    flex: 1;
    width: 100%;
    background-color: #b16666;
    border-radius: 6px;
    overflow: hidden;
    border: 1px dashed white;
  }

  .full-width {
    flex: 999;
  }

  /* -------------------------------------- IA ------------------------------------------- */

  .hidden {
    flex: 0 !important;
    width: 0 !important;
    padding: 0 !important;
    opacity: 0;
    pointer-events: none; /* Empêche de cliquer dessus pendant qu'il est caché */
    border: none !important;
  }

  .tree-wrapper {
    flex: 1;
    opacity: 1;
    overflow: auto;
    box-sizing: border-box;
    border-radius: 6px;

    /* 👉 AJOUTER CETTE LIGNE (Anime le flex, l'opacité et le padding) */
    transition: flex 0.4s cubic-bezier(0.4, 0, 0.2, 1), opacity 0.25s ease-in-out, padding 0.4s ease;
  }

  /* -------------------------------------- IA ------------------------------------------- */



</style>
