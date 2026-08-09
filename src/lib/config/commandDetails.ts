export interface CommandDetail {
  title: string;
  description: string;
  example?: string;
}

export const commandDetails: Record<string, CommandDetail> = {
  "git status": {
    title: "État du projet",
    description:
      "Affiche la liste des fichiers modifiés, suivis ou non suivis.",
    example: "git status",
  },
  "git branch": {
    title: "Indexation globale",
    description: "La commande git branch permet de lister toute les branches",
    example: "git branch",
  },
  "git branch -r": {
    title: "Enregistrer les modifications",
    description:
      "Crée un point de sauvegarde (snapshot) dans l'historique Git.",
    example: 'git commit -m "Mon message"',
  },

  "git branch -a": {
    title: "Enregistrer les modifications",
    description:
      "Crée un point de sauvegarde (snapshot) dans l'historique Git.",
    example: 'git commit -m "Mon message"',
  },
};
