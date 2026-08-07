<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { Terminal } from "@xterm/xterm";
  import { onMount } from "svelte";
  import "@xterm/xterm/css/xterm.css";
  import { spawn } from "tauri-pty";
  import { open } from "@tauri-apps/plugin-dialog";

  import Header from "$lib/components/Header.svelte";
  import GitActionMenu from "$lib/components/gitActionMenu.svelte";
  import GitGraph from "$lib/components/gitGraph.svelte";

  import { type CommitInfo } from "$lib/config/GitActionsMenu";
  import { FitAddon } from "@xterm/addon-fit";


  let terminalElement: HTMLDivElement;

  //on vient stocker la référence vers un div contenu dans la DOM, on ne stocke pas le div en elle même !
  // comme on utilise des package pour le terminal et pour dessiner le git tree on vient indiquer ou les déssiner
  // cela permet d'éviter document.getElementById() car nos composant svelte son réutilisable

  let commits = $state<CommitInfo[]>([]);

  let activeMenu = $state<string | null>(null); 
  let projectPath = $state<string | null>(null); // peut contenir un string (le path du dossier) ou alors null (si aucun dossier fournit) ==> la valeur par défaut est null

  let pty: any = $state(null);
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
        //renderGitGraph(commits);
      }
      else{
        commits = [];
        //gitgraphElement.innerText = "Ce n'est pas un projet git ..."
      }
    }

    catch(error){
      console.error("Erreur lors de la vérification Git:", error);
    }
  }

  onMount(() => {                                           // onMount est une fonction Svelte qui s'execute une seule fois lors de l'initialisation de la page, 
      const term = new Terminal({ 
        cursorBlink: true,
        scrollOnUserInput: true,                            // important à spécifier ca il va descendre automatiquement quand on entre une commande 
      }); 
      
      const fitAddon = new FitAddon();                      // on vient initaliser le "connecteur"
      
      term.loadAddon(fitAddon);                             /*le .loadAddon c'est une méthode du package xterm, 
                                                              elle permet d'ajout des extension/plugin c'est à dire des package secondaire de cette même bibliothèque javascript xtxerm 
                                                              ici en l'occurence on ajoute le plugin fitadon  qu'on a défiint plutot*/

      term.open(terminalElement);                           // on vient injecter le code du package xterm dans terminalElement, (qui pour rappel vient contenir une référence div dans la dom)


      fitAddon.fit();                                       // cette ligne vient calculer le nombre de ligne et de colonne en fonction du <div> 
                                                            // dans lequel, sans cette ligne xterm permet 24 par défaut                    
      
      pty = spawn("bash", [], { cols: term.cols, rows: term.rows });  // on vient générer le programme bash de notre OS, 
                                                                      // [] spécifie les options au démarage du bash ici rien pour un démarage du bash par défaut 
      
      term.onData((data) => pty.write(data));               // quand on vient taper des caractère elles sont dorénavent transmit au pty. il s'active même une fois que la fonction onMount est finei
      pty.onData((data: string) => {                        // onData permet d'écouter l'arrivé de donnée, à l'arrivé on execute une fonction
        term.write(data, () =>{                             // on écrit dans le composant xterm le résutlat renvoyé par le bash
          term.scrollToBottom();                            
        });                                   
      })
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

    <GitGraph
      {activeView}
      {commits}
    />
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

  .terminal-container {
    flex: 1;
    width: 100%;
    background-color: #b16666;
    border-radius: 6px;
    overflow: hidden;
    border: 1px dashed white;
  }

</style>
