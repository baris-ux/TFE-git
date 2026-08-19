export interface CommandDetail {
  title: string;
  description: string;
  example?: string;
  riskLevel?: "safe" | "normal" | "dangereux";
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
      "Crée un point de sauvegarde (snapshot) dans l'historique Git.",
    example: 'git commit -m "Mon message"',
    riskLevel: "safe",
  },
};
