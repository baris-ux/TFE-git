<script lang="ts">
  import { spawn } from "tauri-pty";
  import { Terminal } from "@xterm/xterm";
  import { onMount, tick } from "svelte";
  import { FitAddon } from "@xterm/addon-fit";
  import "@xterm/xterm/css/xterm.css";

  //let terminalElement = $state() as HTMLDivElement | undefined;
  //let pty: any = $state(null);

  let tabs = $state<TerminalLab[]>([]); // cette variable contiendra l'ensemble des instance de terminal ouverts
  let activeTab = $state<string | null>(null); // cette variable contiendra l'instance du terminal sur lequel on se trouve

  interface TerminalLab {
    id: string; //chaque instance de terminal aura son id unique
    name: string; // chaque instance de terminal aura un nom
    element?: HTMLDivElement; // chaque instance aura une référence vers la balise html, on ajoute ? car un terminal non ouvert n'a pas de div dédié
    term?: Terminal;

    fitAddon?: FitAddon; // fitAddon c'est une class contenu dans le package "@xterm/addon-fit", elle définit une structure d'objet et par conséquent on peut l'utiliser comme type
    // il faut que chaque instance a cette clé car chaque terminal a besoin que le nombre de ligne et colonne s'adaptent au div parent

    pty?: any; // chaque xterm aura son pty (bridge) qui permettra de parler avec rust
  }

  async function createTerminalInstance() {
    const newTab: TerminalLab = {
      id: crypto.randomUUID(),
      name: `bash ${tabs.length + 1}`,
    };

    tabs.push(newTab);
    activeTab = newTab.id;

    /* ---------------------------- IA ------------------------------------------------------------------------------------------- */
    await tick();

    const currentTab = tabs.find((t) => t.id === newTab.id);

    if (!currentTab?.element) return;

    const term = new Terminal({
      cursorBlink: true,
      scrollOnUserInput: true,
    });

    const fitAddon = new FitAddon();
    term.loadAddon(fitAddon);

    term.open(currentTab.element);
    fitAddon.fit();

    const pty = spawn("bash", [], { cols: term.cols, rows: term.rows });

    term.onData((data) => pty.write(data));
    pty.onData((data: string) => {
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
    const currentTab = tabs.find((t) => t.id === activeTab);
    if (currentTab?.pty) {
      currentTab.pty.write(`${command}\n`);
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
        <button
          class="tab-button {activeTab === tab.id ? 'active' : ''}"
          onclick={() => {
            activeTab = tab.id;
            tab.fitAddon?.fit();
          }}
        >
          {tab.name}
        </button>
      {/each}
    </div>
    <button onclick={createTerminalInstance}> + Ajouter un bash </button>
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
    /*border-radius: 6px;*/
    flex: 1;
    overflow: hidden;
    border: 1px dashed white;
  }

  .tabs-header {
    display: flex;
    flex-direction: row;
  }

  .tab-button {
    background-color: rgb(221, 207, 191);
    padding: 8px 20px;
    border: none;
    cursor: pointer;
  }

  .tab-button:hover {
    background-color: rgb(119, 102, 83);
  }
  .active {
    background-color: rgb(172, 172, 172);
  }

  .terminal-body {
    flex: 1;
    display: flex;
    flex-direction: column;
  }
</style>
