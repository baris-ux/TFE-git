export interface CommandDetail {
  title: string;
  description: string;
  syntax: string;
  example?: string;
  output?: string;
  riskLevel: "safe" | "normal" | "danger";
  tips?: string;
}

export const commandDetails: Record<string, CommandDetail> = {
  "git status": {
    title: "État du répertoire de travail",
    description: "Affiche l'état des fichiers indexés, modifiés et non suivis.",
    syntax: "git status",
    riskLevel: "safe",
  },

  "git branch": {
    title: "Lister les branches locales",
    description:
      "Affiche la liste des branches locales avec un indicateur sur la branche active.",
    syntax: "git branch",
    output: "* main\n  feature/auth",
    riskLevel: "safe",
  },

  "git branch -r": {
    title: "Lister les branches distantes",
    description:
      "Affiche les branches distantes connues sur les dépôts distants (remotes).",
    syntax: "git branch -r",
    output:
      "  origin/HEAD -> origin/main\n  origin/main\n  origin/feature/auth",
    riskLevel: "safe",
  },

  "git branch -a": {
    title: "Lister toutes les branches (locales et distantes)",
    description:
      "Combine les vues locale et distante pour donner l'inventaire complet des branches accessibles.",
    syntax: "git branch -a",
    output: "* main\n  feature/auth\n  remotes/origin/main",
    riskLevel: "safe",
  },

  "git branch -d": {
    title: "Supprimer une branche locale fusionnée",
    description:
      "Supprime une branche locale en toute sécurité (échoue si les modifications ne sont pas fusionnées).",
    syntax: "git branch -d <nom-branche>",
    example: "git branch -d feature/auth",
    riskLevel: "normal",
    tips: "Utilisez -D pour forcer la suppression même si la branche contient des commits non fusionnés.",
  },

  "git branch -D": {
    title: "Forcer la suppression d'une branche locale",
    description:
      "Supprime immédiatement une branche locale, même si ses commits n'ont pas été fusionnés.",
    syntax: "git branch -D <nom-branche>",
    example: "git branch -D test-abandonne",
    riskLevel: "danger",
    tips: "Attention : les commits non fusionnés deviendront orphelins.",
  },

  "git add": {
    title: "Préparer des fichiers pour le commit",
    description:
      "git add ajoute les modifications sélectionnées à la zone de préparation (staging area). Elles seront incluses dans le prochain commit, mais ne sont pas encore enregistrées dans l’historique Git ni envoyées vers un dépôt distant.",
    syntax: "git add <nom-du-fichier>",
    example: "git add README.txt main.py",
    riskLevel: "safe",
    tips: "Pour save plusieurs fichiers il faut les séparé par un espace",
  },

  "git add .": {
    title: "préparer tous les fichiers à commit",
    description:
      "git add . permet la même chose que git commit mais elle cible tout les fichiers dans le repertoir ou vous êtes situé et de ses sous-dossiers",
    syntax: "git add .",
    riskLevel: "safe",
    tips: "Attention : cette commande ajoute également les fichiers non désirés s'ils ne sont pas listés dans votre fichier .gitignore.",
  },
};
