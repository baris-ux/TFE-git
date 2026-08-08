<script lang="ts">
  import { tutos, type Tutorial } from "$lib/config/tutorials";
  import Terminal from "$lib/components/terminal.svelte";
  import { invoke } from "@tauri-apps/api/core";

  let selectedTutorial = $state<Tutorial | null>(null);
  let currentStepIndex = $state<number>(0);
  let totalAmountOfSteps = $derived(selectedTutorial?.instruction.length ?? 0);

  let resultat = $state<boolean | null>(null);

  async function startTutorial(tuto: Tutorial) {
    selectedTutorial = tuto;
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
    resultat = null;
  }
</script>

<main class="sandbox-container">
  <div class="content-wrapper">
    {#if selectedTutorial === null}
      <a href="/app" class="back-button">Retour</a>

      <header>
        <h1>Mode Bac à Sable ⛱️</h1>
        <p class="subtitle">Sélectionnez un module</p>
      </header>

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
    {:else}
      <div class="tutorial-lesson-container">
        <header class="exercise-header">
          <h1>{selectedTutorial.title}</h1>
          <p class="subtitle">{selectedTutorial.description}</p>
        </header>

        <div class="exercise-workspace">
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

          <button class="quit-btn" onclick={quitTutorial}>
            Choisir un autre tutoriel
          </button>
        </div>

        {#if resultat === false}
          <p class="error-message">l'étape n'est pas validé</p>
        {:else if resultat === true}
          <p class="sucess-message">Correct</p>
        {/if}
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

  .tuto-card {
    background: #313244;
    border: 1px solid #45475a;
    border-radius: 12px;
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
    border-radius: 12px;
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

  .exercise-workspace {
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
  }

  .validate-btn {
    background-color: #379792;
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
    background-color: rgb(207, 101, 101);
    padding: 10px;
    border-radius: 5px;
  }
</style>
