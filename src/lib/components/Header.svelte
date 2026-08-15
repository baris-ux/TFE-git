<script lang="ts">
  import { resolve } from "$app/paths";

  let { projectPath, toggleView, selectProject } = $props();
  // on créeer une liason unidirectionnel (du parent vers l'enfant) et pour ca on utilise un props qui s'écrit de la manière suivant let {} = $props
  // ca permet de pointer les variable et fonctions existant dans le parent pour les utilsier directement dans ce fichier
  // par défaut ils on ne peut pas changer leur valeur
</script>

<header class="header-bar">
  <div>
    <h1>Actions guidées</h1>
    <p>
      Projet actuel : <span class="project-path"
        >{projectPath ?? "aucun projet sélectionné"}</span
      >
    </p>
  </div>

  <div class="button-list">
    <button class="toggle-btn" onclick={() => toggleView("actions")}>
      plein écran menu de sélection
    </button>

    <button class="toggle-btn" onclick={() => toggleView("tree")}>
      plein écran graphe visuel
    </button>

    <button class="open-btn" onclick={selectProject}>
      Ouvrir un projet Git
    </button>

    <a href={resolve("/sandbox")} class="sandbox-btn"> mode bac à sable ⛱️ </a>
    <!-- on utilise resolve, il permet d'aider à générer des type les chemins internes -->
    <!-- même si on spéciie le chemin, elle utilise les route généré par le file-based-routing pour généré des url-->
    <!-- à noter que resolveRoute est déprécié, c'est à dire qu'elle fonctionne mais qu'elle sera obselète à l'avenir-->
  </div>
</header>

<style>
  h1,
  p {
    color: white;
    font-weight: bold;
    font-family: "Inter", sans-serif;
    margin-top: 0;
    margin-bottom: 15px;
  }
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

  .button-list {
    display: flex;
    flex-direction: row;
    gap: 15px;
  }

  .open-btn,
  .toggle-btn,
  .sandbox-btn {
    background-color: #2c539e;
    color: white;
    border: none;
    padding: 10px 16px;
    font-size: 0.95rem;
    font-weight: bold;
    border-radius: 4px;
    cursor: pointer;
    text-decoration: none;
    transition: background-color 0.2s;
  }

  .open-btn:hover,
  .toggle-btn:hover {
    background-color: #3b69c4;
  }
</style>
