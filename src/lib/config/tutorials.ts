export interface Tutorial {
  id: string;
  title: string;
  description: string;
  difficulty: "Débutant" | "Intermédiaire" | "Avancé";
  instruction: string[];
}

export const tutos: Tutorial[] = [
  {
    id: "exo-1",
    title: "1. Mon Premier Commit",
    description:
      "Apprenez à ajouter des fichiers dans la Staging Area et créer un commit.",
    difficulty: "Débutant",
    instruction: [
      "1. Pour commencer, rendez-vous sur votre bureau cd bureau et créez y un dossier nommé test",
      "2. puis entrez la commande git init",
      "3. et voila !",
    ],
  },
];
