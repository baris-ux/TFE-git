<script lang="ts">
  import { spawn } from "tauri-pty";
  import { Terminal } from "@xterm/xterm";
  import { onMount, tick } from "svelte";
  import { FitAddon } from "@xterm/addon-fit";
  import "@xterm/xterm/css/xterm.css";

  let tabs = $state<TerminalLab[]>([]); // cette variable contiendra l'ensemble des instance de terminal ouverts
  let activeTab = $state<string | null>(null); // cette variable contiendra l'ID de l'instance du terminal !!!
  let { loadGitHistory } = $props();

  interface TerminalLab {
    id: string; //chaque instance de terminal aura son id unique
    name: string; // chaque instance de terminal aura un nom
    element?: HTMLDivElement; // chaque instance aura une référence vers la balise html, on ajoute ? car un terminal non ouvert n'a pas de div dédié
    term?: Terminal;

    fitAddon?: FitAddon; // fitAddon c'est une class contenu dans le package "@xterm/addon-fit", elle définit une structure d'objet et par conséquent on peut l'utiliser comme type
    // il faut que chaque instance a cette clé car chaque terminal a besoin que le nombre de ligne et colonne s'adaptent au div parent

    pty?: any; // chaque xterm aura son pty (bridge) qui permettra de parler avec rust
  }

  async function selectTab(id: string) {
    activeTab = id;
    await tick();
    const currentTab = tabs.find((t) => t.id === id);
    currentTab?.fitAddon?.fit();
  }

  function deleteTab(tabId: string) {
    let indexTabId = tabs.findIndex((tab) => tab.id === tabId); // trouve l'indice de l'objet ou l'id est égale à l'id du tab qu'on souhaite delete
    tabs.splice(indexTabId, 1); // splice prend comme paramètre l'index et le nombre d'élément à delete dans la liste
  }

  async function createTerminalInstance() {
    const newTab: TerminalLab = {
      id: crypto.randomUUID(),
      name: `bash ${tabs.length + 1}`,
    };

    tabs.push(newTab);
    activeTab = newTab.id;

    await tick();

    /* cette ligne de code bien que simple fut très complexe à comprendre, je vais prendre le temps de vous l'expliquer
    await tick() est une fonction qui permet d'imterrompre l'execution pendant quelque seconde avant de reprendre son execution
    mais alors pourquoi interrompre l'execution de la fonction createTerminalInstance ? c'est parce qu'on vient de push un objet tab dans a list tabs
    et dans le html on va venir boucler sur cette liste pour déssiner ce tab
    
    En fait il faut que notre fonction soit en pause le temps d'un instant pour qu'on vient déssiner ce nouveau tab avant de poursuivre l'execution de la fonction
    car le code qui vient a besoin qu'il soit déssiner pour qu'il s'execute correctement*/

    const currentTab = tabs.find((t) => t.id === newTab.id);

    /* currentTab viendra contenir le terminal en elle même de ce tab sous la forme d'un objet unique 

    exemple : 

    {
      "id": "3f8a...",
      "name": "Terminal 2",
      "pty": { ... },
      "term": { ... }
    }

    la fonction find() permet de parcourir une liste d'objet
    il vérifie pour chaque objet parcouru si l'id de l'objet parcouru équivaut à l'id de newTab à savoir le dernier tab crée
    find() renvoie l'objet elle même si la condition === est true */

    if (!currentTab?.element) return; //si l'objet currentTab (le terminal) n'a pas l'attrbit/clé element ('c'est qu'il n'est pas déssiné par svelte dans la DOM
    // ) dans ce cas on arrête l'execution de la fonction avec un return

    /* ---------------------------- IA ---------------------------------------------------------------------------------------------------*/

    const term = new Terminal({
      // on définit le terminal avec le fonction new Terminal
      cursorBlink: true,
      scrollOnUserInput: true,
    });

    const fitAddon = new FitAddon();
    term.loadAddon(fitAddon);

    term.open(currentTab.element);
    fitAddon.fit();

    const pty = spawn("bash", [], { cols: term.cols, rows: term.rows });

    term.onData((data) => {
      pty.write(data);

      if (data === "\r") {
        setTimeout(() => {
          loadGitHistory?.();
        }, 400);
      }
    });

    pty.onData((data: Uint8Array) => {
      term.write(data, () => {
        term.scrollToBottom();
      });
    });

    currentTab.term = term;
    currentTab.fitAddon = fitAddon;
    currentTab.pty = pty;

    /* ---------------------------- IA ---------------------------------------------------------------------------------------------------*/
  }

  export function sendCommand(command: string) {
    const currentTab = tabs.find((t) => t.id === activeTab); // dans l'array do'bjet tabs cherche une corresspondence entre l'id de l'objet et activeTab (c'est un id)
    if (currentTab?.pty) {
      // si currentTab existe ET que l'attribut pty existe
      currentTab.pty.write(`${command}\n`);

      setTimeout(() => {
        if (loadGitHistory) {
          loadGitHistory();
        }
      }, 400);
    }
  }

  /*
  export function sendCommand(command: string) {
    if (pty) {
      pty.write(`${command}\n`);
    }
  } */

  /*function createTerminal() {
    if (!terminalElement) return;

    // onMount est une fonction Svelte qui s'execute une seule fois lors de l'initialisation de la page,
    const term = new Terminal({
      cursorBlink: true,
      scrollOnUserInput: true, // important à spécifier ca il va descendre automatiquement quand on entre une commande
    });

    const fitAddon = new FitAddon(); // on vient initaliser le "connecteur"

    term.loadAddon(fitAddon); //le .loadAddon c'est une méthode du package xterm,
    //elle permet d'ajout des extension/plugin c'est à dire des package secondaire de cette même bibliothèque javascript xtxerm
    //ici en l'occurence on ajoute le plugin fitadon  qu'on a défiint plutot

    term.open(terminalElement); // on vient injecter le code du package xterm dans terminalElement, (qui pour rappel vient contenir une référence div dans la dom)

    fitAddon.fit(); // cette ligne vient calculer le nombre de ligne et de colonne en fonction du <div>
    // dans lequel, sans cette ligne xterm permet 24 par défaut

    pty = spawn("bash", [], { cols: term.cols, rows: term.rows }); // on vient générer le programme bash de notre OS,
    // [] spécifie les options au démarage du bash ici rien pour un démarage du bash par défaut

    term.onData((data) => pty.write(data)); // quand on vient taper des caractère elles sont dorénavent transmit au pty. il s'active même une fois que la fonction onMount est finei
    pty.onData((data: string) => {
      // onData permet d'écouter l'arrivé de donnée, à l'arrivé on execute une fonction
      term.write(data, () => {
        // on écrit dans le composant xterm le résutlat renvoyé par le bash
        term.scrollToBottom();
      });
    });
  } */

  onMount(() => {
    createTerminalInstance();
  });
</script>

<div class="terminal-wrapper">
  <div class="terminal-toolbar">
    <div class="tabs-header">
      <!-- on vient boucler sur la liste tabs pour afficher tout les tab disponible dans la liste, chacun des éléments s'appelant tab -->
      {#each tabs as tab (tab.id)}
        <div
          class="tab"
          class:active={tab.id === activeTab}
          onclick={() => (activeTab = tab.id)}
          // pour se débarasser du warning, car un <div> n'est pas censé être cliquable
          role="button"
          tabindex="0"
          onkeydown={(e) =>
            (e.key === "Enter" || e.key === " ") && selectTab(tab.id)}
        >
          <span>{tab.name}</span>
          <button class="close-btn" onclick={() => deleteTab(tab.id)}>
            &times;
          </button>
        </div>
      {/each}
    </div>
    <button class="add-tab-button" onclick={createTerminalInstance}>
      + Ajouter un bash
    </button>
  </div>

  <!-- IA -->

  <div class="terminal-body">
    {#each tabs as tab (tab.id)}
      <div
        bind:this={tab.element}
        class="terminal-container"
        style:display={activeTab === tab.id ? "block" : "none"}
      ></div>
    {/each}
  </div>
  <!-- IA -->

  <!--<div bind:this={tab.element} class="terminal-container"></div> -->
</div>

<style>
  .terminal-wrapper {
    width: 100%;
    height: 25%;
    display: flex;
    flex-direction: column;
  }

  .terminal-toolbar {
    display: flex;
    flex-direction: row;
    background-color: rgb(78, 78, 78);
  }

  .terminal-container {
    flex: 1;
    overflow: hidden;
    border: 1px dashed white;
  }

  .tabs-header {
    display: flex;
    flex-direction: row;
  }

  .terminal-body {
    flex: 1;
    display: flex;
    flex-direction: column;
  }

  .tab,
  .add-tab-button {
    background-color: #252526;
    color: #999999;
    padding: 8px 16px;
    border: none;
    transition:
      background-color 0.15s ease,
      color 0.15s ease;
    cursor: pointer;
  }

  .tab:hover,
  .add-tab-button {
    background-color: #2a2d2e;
    color: #cccccc;
  }
  .active {
    background-color: #1e1e1e;
    color: #ffffff;
    border-top: 2px solid #007acc;
  }

  .close-btn {
    border: none;
    cursor: pointer;
    color: #888888;
    border-radius: 4px;
    padding: 2px 6px;
    font-size: 14px;
    transition: all 0.15s ease;
  }

  .close-btn:hover {
    background-color: #c72e2e;
    color: #ffffff;
  }
</style>
