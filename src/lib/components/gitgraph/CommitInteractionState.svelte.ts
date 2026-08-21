import { invoke } from "@tauri-apps/api/core";
import type { CommitInfo } from "$lib/config/GitActionsMenu";

export class CommitInteractionState {
  isComparaisonActive = $state(false);
  firstHash = $state<string | null>(null);
  secondHash = $state<string | null>(null);
  selectedCommit = $state<CommitInfo | null>(null); // cette variable va contenir l'objet CommitInfo qu'on a définit dans GitActionMenu.ts
  // il va stocker les différent information du noeud sur lequel on va cliquer sur un noeud
  diffError = $state<string | null>(null);
  isBarActive = $state(false); // c'est le menu d'intéraction qu'on met à faux par défaut pour qu'il ne soit pas affiché
  commitInfoDisplayed = $state(false);
  diffResult = $state<string | null>(null);

  openBoxOnCommitClick(commit: CommitInfo, path: string | null) {
    // on passe en paramètre l'objet CommitInfo définit dans le config/GitActionsMenu.ts
    if (this.isComparaisonActive) {
      if (commit.id === this.firstHash) {
        this.diffError = "Choisis un commit différent du premier.";
        return;
      }
      this.secondHash = commit.id;
      this.commitComparaison(path);
      return;
    }

    this.selectedCommit = commit;
    this.commitInfoDisplayed = false;
    this.isBarActive = true;
  }

  displayCommitInfo() {
    this.commitInfoDisplayed = true;
    this.isBarActive = false;
  }

  startComparaison() {
    if (!this.selectedCommit) return;
    // si selectedCommit est un objet renvoit false
    // si c'est false on return  pour arrêter toute de suite l'execution de la fonction
    this.firstHash = this.selectedCommit.id; // on récupère le hash du commit déja selectionné (lorsqu'on a cliquer pour faire apparaitre le menu)
    this.secondHash = null; // on set toute les valeur à null
    this.diffResult = null;
    this.diffError = null;
    this.isBarActive = false; // on ferme le menu d'intéraction
    this.isComparaisonActive = true;
  }

  cancelComparaison() {
    this.isComparaisonActive = false;
    this.firstHash = null;
    this.secondHash = null;
  }

  async commitComparaison(path: string | null) {
    if (this.firstHash === null || this.secondHash === null || path === null)
      return;
    this.diffError = null;
    try {
      this.diffResult = await invoke<string>("compare_commit", {
        path,
        oldCommit: this.firstHash,
        newCommit: this.secondHash,
      });
    } catch (err) {
      this.diffError = String(err);
    }
  }
}
