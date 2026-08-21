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
  is_head: boolean;
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
      { label: "supprimer une branche local", command: "git branch -d" },
    ],
  },
  {
    label: "Ajouter des modifications",
    command: "git add",
    subMenu: [
      { label: "Ajouter un fichier", command: "git add " },
      { label: "Ajouter toutes les modifications", command: "git add ." },
    ],
  },
];

/* on définit une fonction qui lorsqu'on l'appelle donne qui inverse la valeur de la variable showBranchMenu 
     l'inverse de faux ==> vrai */
