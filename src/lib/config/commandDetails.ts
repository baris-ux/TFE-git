export interface CommandDetail {
  title: string;
  description: string;
  example?: string;
  riskLevel?: "safe" | "normal" | "danger";
  output?: string;
}

export const commandDetails: Record<string, CommandDetail> = {
  "git status": {
    title: "État du projet",
    description:
      "Affiche la liste des fichiers modifiés, suivis ou non suivis.",
    example: "git status",
    riskLevel: "safe",
  },
  "git branch": {
    title: "Indexation globale",
    description:
      "La commande git branch permet de lister toute les branches en local sur votre machine, son execution est tout à fait sans danger",
    example: "git branch",
    riskLevel: "safe",
  },
  "git branch -r": {
    title: "lister toutes les branches distant",
    description:
      "La commande git branch -r permet de lister toutes les branches sur votre répo distant connu par votre projet en local",
    example: "git branch -r",
    riskLevel: "safe",
  },

  "git branch -a": {
    title: "Enregistrer les modifications",
    description:
      "Affiche les branches locales ainsi que les branches de suivi à distance connues par le dépôt local.",
    example: "git branch -a",
    output:
      "* main\n  feature/login\n  remotes/origin/main\n  remotes/origin/feature/",
    riskLevel: "safe",
  },

  "git branch -d": {
    title: "Supprimer une branche locale de force",
    description:
      "Supprime une branche locale, même si son contenu n’a pas encore été fusionné dans une autre branche.",
    example: "git branch -d <nom-branche>",
    riskLevel: "danger",
  },
};
