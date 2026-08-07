<script lang="ts">
  import { spawn } from "tauri-pty";
  import { Terminal } from "@xterm/xterm";
  import { onMount } from "svelte";
  import { FitAddon } from "@xterm/addon-fit";
  import "@xterm/xterm/css/xterm.css";

  let terminalElement = $state() as HTMLDivElement | undefined;
  let pty: any = $state(null);

  // Exposer la méthode pour le composant parent (+page.svelte)
  export function sendCommand(command: string) {
    if (pty) {
      pty.write(`${command}\n`);
    }
  }

  function createTerminal() {
    if (!terminalElement) return;

    // onMount est une fonction Svelte qui s'execute une seule fois lors de l'initialisation de la page,
    const term = new Terminal({
      cursorBlink: true,
      scrollOnUserInput: true, // important à spécifier ca il va descendre automatiquement quand on entre une commande
    });

    const fitAddon = new FitAddon(); // on vient initaliser le "connecteur"

    term.loadAddon(
      fitAddon,
    ); /*le .loadAddon c'est une méthode du package xterm, 
                                                                    elle permet d'ajout des extension/plugin c'est à dire des package secondaire de cette même bibliothèque javascript xtxerm 
                                                                    ici en l'occurence on ajoute le plugin fitadon  qu'on a défiint plutot*/

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
  }

  onMount(() => {
    createTerminal();
  });
</script>

<div class="terminal-wrapper">
  <div class="terminal-toolbar">
    <button>+ Ajouter un bash</button>
  </div>

  <div bind:this={terminalElement} class="terminal-container"></div>
</div>

<style>
  .terminal-wrapper {
    width: 100%;
    height: 25%;
    display: flex;
    flex-direction: column;
  }

  .terminal-toolbar {
    background-color: rgb(78, 78, 78);
  }

  .terminal-container {
    /*background-color: #b16666;*/
    border-radius: 6px;
    flex: 1;
    overflow: hidden;
    border: 1px dashed white;
  }
</style>
