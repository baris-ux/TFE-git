export interface Exercice {
  id: string;
  title: string;
  description: string;
  difficulty: "Débutant" | "Intermédiaire" | "Avancé";
  setupCommands: string[][]; // [][] pour tableau de tableau
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
      ["git", "init", "-b", "main"],
      ["touch", "README.md"], // <-- 1. Crée le fichier sur Linux/macOS
      ["git", "add", "README.md"], // <-- 2. Maintenant Git le trouve !
      ["git", "commit", "-m", "Initial commit"],
      ["git", "checkout", "-b", "feature"],
      ["touch", "feature.txt"], // <-- Crée un fichier pour la branche feature
      ["git", "add", "."],
      ["git", "commit", "-m", "feat: nouvelle option"],
      ["git", "checkout", "main"],
    ],
    instruction: [
      "Assurez-vous d'être placé sur la branche 'main'.",
      "Fusionnez la branche 'feature' dans votre branche actuelle.",
    ],
  },
];
