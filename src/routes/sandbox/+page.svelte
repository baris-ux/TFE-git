<script lang="ts">
  import { tutos, type Tutorial } from "$lib/config/tutorials";
  import { exercices, type Exercice } from "$lib/config/exercices";
  import Terminal from "$lib/components/terminal.svelte";
  import { invoke } from "@tauri-apps/api/core";

  let selectedTutorial = $state<Tutorial | null>(null);
  let currentStepIndex = $state<number>(0);
  let totalAmountOfSteps = $derived(selectedTutorial?.instruction.length ?? 0);

  let selectedExercice = $state<Exercice | null>(null);

  let resultat = $state<boolean | null>(null);

  async function startTutorial(tuto: Tutorial) {
    selectedTutorial = tuto;
  }

  async function startExercise(exercice: Exercice) {
    console.log("1. Bouton cliqué ! Exercice :", exercice.id);
    selectedExercice = exercice;

    try {
      console.log("2. Envoi de la commande à Rust...");
      const folderPath = await invoke<string>("setup_exercise_repo", {
        setupCommands: exercice.setupCommands,
        id: exercice.id,
      });

      console.log("3. Succès ! Dossier créé à :", folderPath);
    } catch (error) {
      console.error("❌ Erreur côté Rust :", error);
    }
  }

  async function answerVerification() {
    if (!selectedTutorial) return;

    resultat = await invoke<boolean>("verify_tutorial_step", {
      tutorialId: selectedTutorial.id,
      stepIndex: currentStepIndex,
    });
  }

  function nextStep() {
    if (resultat === true && currentStepIndex < totalAmountOfSteps) {
      currentStepIndex += 1;
      resultat = null; // pour éviter que cela soit marqué comme vrai quand on se rend à la prochaine question
    }
  }

  function quitTutorial() {
    selectedTutorial = null;
    selectedExercice = null;
    resultat = null;
  }
</script>

<main class="sandbox-container">
  <div class="content-wrapper">
    <!-- SI on a ni selectionné un tutoriel ni un exercice on vient afficher le menu de la page-->
    {#if selectedTutorial === null && selectedExercice === null}
      <a href="/app" class="back-button">Retour</a>

      <header>
        <h1>Mode Bac à Sable ⛱️</h1>
        <p class="subtitle">Sélectionnez un module</p>
      </header>

      <div class="module-selection">
        <h2>Tutoriels Guidés</h2>
        <div class="tutorial-grid">
          <!-- on vient parcourir un array d'objet -->
          {#each tutos as tuto (tuto.id)}
            <!-- on passe l'objet tuto en paramètre-->
            <button class="tuto-card" onclick={() => startTutorial(tuto)}>
              <div class="card-header">
                <span class="badge {tuto.difficulty.toLowerCase()}"
                  >{tuto.difficulty}</span
                >
              </div>
              <h3>{tuto.title}</h3>
              <p>{tuto.description}</p>
            </button>
          {/each}
        </div>

        <h2>Exercice</h2>
        <div class="exercice-grid">
          {#each exercices as exo (exo.id)}
            <button class="exercice-card" onclick={() => startExercise(exo)}>
              <div class="card-header">
                <span class="badge {exo.difficulty.toLowerCase()}">
                  {exo.difficulty}
                </span>
              </div>
              <h3>{exo.title}</h3>
              <p>{exo.description}</p>
            </button>
          {/each}
        </div>
      </div>

      <!-- si on a cliqué sur un tutoriel on va venir l'affiché-->
    {:else if selectedTutorial !== null}
      <div class="tutorial-lesson-container">
        <header class="exercise-header">
          <h1>{selectedTutorial.title}</h1>
          <p class="subtitle">{selectedTutorial.description}</p>
        </header>

        <div class="tutorial-workspace">
          {#if currentStepIndex < totalAmountOfSteps}
            <p class="subtitle">
              {selectedTutorial.instruction[currentStepIndex]}
            </p>
          {:else}
            <p>
              fécilication vous avez finit le tutoriiel {selectedTutorial.title}
            </p>
            <button class="">voir un autre module</button>
          {/if}
        </div>

        <Terminal />

        {#if resultat === false}
          <p class="error-message">L'étape n'est pas validée. Réessayez !</p>
        {:else if resultat === true}
          <p class="success-message">Étape validée avec succès !</p>
        {/if}

        <div class="button-below-exercice">
          {#if resultat === true}
            <button class="validate-btn" onclick={() => nextStep()}>
              continuer
            </button>
          {:else}
            <button class="validate-btn" onclick={() => answerVerification()}>
              {resultat === false ? "réesayer" : "valider"}
              <!-- si la résultat à pour valeur false si oui le texte sera sinon valider-->
            </button>
          {/if}

          <button class="quit-btn" onclick={quitTutorial}> Quitter </button>
        </div>
      </div>
      <!-- s'affiche uniquement si on à choisit un exercice -->
    {:else if selectedExercice !== null}
      <div class="exercice-container">
        <h1>{selectedExercice.title}</h1>
        <Terminal />

        <div class="button-below-exercice">
          <button class="quit-btn" onclick={() => (selectedExercice = null)}>
            Quitter
          </button>
        </div>
      </div>
    {/if}
  </div>
</main>

<style>
  :global(body) {
    margin: 0;
  }

  .sandbox-container {
    padding: 40px 20px;
    background: #1e1e2e;
    min-height: 100vh;
    box-sizing: border-box;
    color: #cdd6f4;
  }

  .module-selection {
    /* le conteneur qui contient l'ensemble du menu avec tuto et exercices */
    display: flex;
    flex-direction: column;
    gap: 32px;
  }

  .tutorial-lesson-container {
    display: flex;
    flex-direction: column;
    gap: 15px;
  }

  .content-wrapper {
    max-width: 900px;
    margin: 0 auto;
  }

  h1 {
    color: #ffffff;
    margin: 0 0 8px 0;
  }

  .subtitle {
    color: #a6adc8;
    margin-top: 0;
    margin-bottom: 32px;
  }

  .tutorial-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(260px, 1fr));
    gap: 20px;
  }

  .tuto-card,
  .exercice-card {
    background: #313244;
    border: none;
    padding: 20px;
    text-align: left;
    cursor: pointer;
    transition: all 0.2s ease;
    display: flex;
    flex-direction: column;
    justify-content: space-between;
  }

  .tuto-card:hover {
    transform: translateY(-4px);
    border-color: #89b4fa;
    background: #3b3d54;
    border: none;
    box-shadow: 0 8px 20px rgba(0, 0, 0, 0.3);
  }

  .exercice-card {
    border-left: 4px solid #fab387;
  }

  .exercice-card:hover {
    transform: translateY(-4px);
    border: none;
    box-shadow: 0 8px 20px rgba(0, 0, 0, 0.3);
  }

  .card-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 12px;
  }

  .badge {
    font-size: 0.75rem;
    padding: 4px 8px;
    /*border-radius: 12px;*/
    font-weight: bold;
  }

  .badge.débutant {
    background: #a6e3a1;
    color: #11111b;
  }

  .badge.intermédiaire {
    background: #f9e2af;
    color: #11111b;
  }

  .badge.avancé {
    background: #f38ba8;
    color: #11111b;
  }

  h3 {
    color: #f5e0dc;
    margin: 0 0 8px 0;
    font-size: 1.1rem;
  }

  p {
    color: #bac2de;
    font-size: 0.88rem;
    line-height: 1.4;
    margin: 0 0 16px 0;
  }

  .action-text {
    color: #89b4fa;
    font-weight: bold;
    font-size: 0.85rem;
  }

  .tutorial-workspace {
    background: #313244;
    border: 1px solid #45475a;
    border-radius: 12px;
    padding: 30px;
    text-align: center;
  }

  .button-below-exercice {
    display: flex;
    flex-direction: row;
    gap: 10px;
    align-items: center;
  }

  .quit-btn {
    background-color: #f38ba8;
    color: #11111b;
    font-weight: bold;
  }

  .validate-btn {
    background-color: #379792;
    color: #ffffff;
    font-weight: bold;
  }

  .validate-btn,
  .quit-btn {
    cursor: pointer;
    padding: 10px 20px;
    margin-top: 10px;
    border: none;
    border-radius: 5px;
  }

  .back-button {
    background-color: rgb(190, 190, 190);
    border: none;
    padding: 10px 24px;
    text-decoration: none;

    transition:
      background-color 0.25s ease,
      border-color 0.25s ease,
      color 0.25s ease;
  }

  .back-button:hover {
    background-color: #fff7d1;
  }

  .error-message {
    background-color: rgba(243, 139, 168, 0.2);
    border: 1px solid #f38ba8;
    color: #f38ba8;
    padding: 12px;
    border-radius: 6px;
    margin: 0;
    font-weight: bold;
  }

  .success-message {
    background-color: rgba(166, 227, 161, 0.2);
    border: 1px solid #a6e3a1;
    color: #a6e3a1;
    padding: 12px;
    border-radius: 6px;
    margin: 0;
    font-weight: bold;
  }
</style>
