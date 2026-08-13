<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { openUrl } from "@tauri-apps/plugin-opener";

  let isGitInstalled = $state<boolean | null>(null);

  async function verification_if_git_installed() {
    isGitInstalled = await invoke<boolean>("verify_if_git_installed");
  }

  async function openGitWebsites(event: MouseEvent) {
    event.preventDefault();
    await openUrl("https://git-scm.com/install/");
  }
</script>

<main>
  <h1>
    Bienvenue sur git edutool de commencer vérifions si vous avez git sur votre
    machine
  </h1>

  {#if isGitInstalled === null}
    <button class="verification-btn" onclick={verification_if_git_installed}>
      vérifier si git est présent
    </button>
  {:else if isGitInstalled === true}
    <p class="success">git est bien présent sur votre machine !</p>
    <a href="/app" class="start-btn">Commencez !</a>
  {:else if isGitInstalled === false}
    <div class="container-fail-case">
      <p class="error">git n'est pas présent sur votre machine ...</p>
      <button class="verification-btn" onclick={verification_if_git_installed}>
        revérifier
      </button>
      <a
        href="https://git-scm.com/install"
        class="redirection-btn"
        onclick={openGitWebsites}
      >
        installer git
      </a>
    </div>
  {/if}
</main>

<style>
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

  .container-fail-case {
    display: flex;
    flex-direction: column;
    gap: 15px;
  }

  .start-btn,
  .redirection-btn,
  .verification-btn {
    color: white;
    border: none;
    padding: 12px 24px;
    cursor: pointer; /* ai je vraiment besoin d'expliquer ... */
    text-decoration: none; /* pour retirer le soulignement */
  }
  .start-btn {
    background-color: #2ea043;
  }
  .verification-btn {
    background-color: rgb(219, 71, 71);
  }
  .redirection-btn {
    background-color: rgb(46, 76, 160);
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
