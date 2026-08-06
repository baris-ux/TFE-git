/*  
interfaceCommitInfo
indique dans notre code svelte à quoi doit ressembler une un "objet" 
commit avec ses clé et le type de valeur pour chaque clé 
il ne contient pas d'information en tant que telle  
*/

// src/lib/config/gitActions.ts

export interface CommitInfo {
  id: string;
  message: string;
  author: string;
  parents: string[];
  branches: string[];
}

export interface SubMenu {
  label: string;
  command: string;
}

export interface GitAction {
  label: string;
  command: string;
  subMenu?: SubMenu[];
}

export const dropdownGitActions: GitAction[] = [
  {
    label: "Afficher branches",
    command: "git branch",
    subMenu: [
      { label: "branche local", command: "git branch" },
      { label: "branche distant", command: "git branch -r" },
      { label: "branche local + distant", command: "git branch -a" },
    ],
  },
  {
    label: "Changer de branche",
    command: "git checkout",
    subMenu: [
      { label: "basculer sur une branche existante", command: "git checkout" },
      { label: "Crée une nouvelle branche et basculer dessus", command: "git branch -b" },
      { label: "basculer sur la dernière branche ou tu te trouvrais", command: "git checkout -" }
    ],
  },
  {
    label: "git status",
    command: "git commit",
    subMenu: [
      { label: "Statut détaillé, par défaut", command: "git status" },
      { label: "Statut compact", command: "git status -s" },
    ],
  },
  {
    label: "Envoyer les modifications",
    command: "git push",
    subMenu: [
      { label: "envoyer modif", command: "git push" },
      { label: "Publier une nouvelle branche", command: "git push -u" },
    ],
  },
];

  /* on définit une fonction qui lorsqu'on l'appelle donne qui inverse la valeur de la variable showBranchMenu 
     l'inverse de faux ==> vrai */