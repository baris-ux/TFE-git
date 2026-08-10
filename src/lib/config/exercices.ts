export interface Exercice {
  id: string;
  title: string;
  description: string;
  difficulty: "Débutant" | "Intermédiaire" | "Avancé";
  setupCommands: string[];
  instruction: string[];
}

export const exercices: Exercice[] = [
  {
    id: "exo-fusion-simple",
    title: "Fusionner une branche",
    description:
      "Rapatriez le travail terminé de la branche 'feature' vers la branche principale 'main'.",
    difficulty: "Débutant",
    setupCommands: [
      "git init",
      "git add README.md", // Le fichier aura été créé par Rust juste avant !
      "git commit -m 'Initial commit'",
      "git checkout -b feature",
      "git commit -am 'feat: nouvelle option'",
      "git checkout main",
    ],
    instruction: [
      "Assurez-vous d'être placé sur la branche 'main'.",
      "Fusionnez la branche 'feature' dans votre branche actuelle.",
    ],
  },
];
