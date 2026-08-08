<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  let isGitInstalled = $state<boolean | null>(null);

  async function verification_if_git_installed() {
    isGitInstalled = await invoke<boolean>("verify_if_git_installed");
  }
</script>

<main>
  <h1>
    Bienvenue sur git edutool de commencer vérifions si vous avez git sur votre
    machine
  </h1>

  <button onclick={verification_if_git_installed}
    >vérifier si git est présent</button
  >
  {#if isGitInstalled === true}
    <p class="success">git est bien présent sur votre machine !</p>
    <a href="/app" class="start-btn">Commencez !</a>
  {:else if isGitInstalled === false}
    <p class="error">git n'est pas présent sur votre machine ...</p>
  {/if}
</main>

<style>
  :global(body) {
    margin: 0;
    font-family:
      system-ui,
      -apple-system,
      sans-serif;
    background-color: #0d1117;
    color: #c9d1d9;
  }

  main {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    min-height: 100vh;
    padding: 20px;
    box-sizing: border-box;
    text-align: center;
  }

  h1 {
    font-size: 1.5rem;
    max-width: 600px;
    margin-bottom: 24px;
    color: #f0f6fc;
    line-height: 1.4;
  }

  button {
    background-color: #238636;
    color: #ffffff;
    border: none;
    padding: 12px 24px;
    font-size: 1rem;
    font-weight: 600;
    border-radius: 6px;
    cursor: pointer;
    transition: background-color 0.2s ease;
  }

  button:hover {
    background-color: #2ea043;
  }

  .start-btn {
    background-color: #2ea043;
    color: white;
    border: none;
    padding: 12px 24px;
    cursor: pointer; /* ai je vraiment besoin d'expliquer ... */
    text-decoration: none; /* pour retirer le soulignement */
  }

  p {
    margin-top: 20px;
    padding: 12px 20px;
    border-radius: 6px;
    font-weight: 500;
  }

  p.success {
    background-color: rgba(46, 160, 67, 0.15);
    color: #3fb950;
    border: 1px solid rgba(46, 160, 67, 0.4);
  }

  p.error {
    background-color: rgba(248, 81, 73, 0.15);
    color: #f85149;
    border: 1px solid rgba(248, 81, 73, 0.4);
  }
</style>
