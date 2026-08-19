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
  {
    label: "Changer de branche",
    command: "git checkout",
    subMenu: [
      { label: "basculer sur une branche existante", command: "git checkout" },
      {
        label: "Crée une nouvelle branche et basculer dessus",
        command: "git branch -b",
      },
      {
        label: "basculer sur la dernière branche ou tu te trouvrais",
        command: "git checkout -",
      },
    ],
  },
  {
    label: "afficher le statut",
    command: "git status",
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
  {
    label: "Créer un commit",
    command: "git commit",
    subMenu: [
      { label: "Créer un commit avec un message", command: 'git commit -m ""' },
      { label: "Modifier le dernier commit", command: "git commit --amend" },
    ],
  },
  {
    label: "Consulter l'historique",
    command: "git log",
    subMenu: [
      { label: "Historique détaillé", command: "git log" },
      {
        label: "Historique compact avec graphe",
        command: "git log --oneline --graph --all",
      },
    ],
  },
  {
    label: "Synchroniser avec le dépôt distant",
    command: "git pull",
    subMenu: [
      {
        label: "Récupérer et intégrer les modifications distantes",
        command: "git pull",
      },
      {
        label: "Récupérer les références sans modifier la branche",
        command: "git fetch",
      },
      { label: "Envoyer les commits", command: "git push" },
      { label: "Publier une nouvelle branche", command: "git push -u origin " },
    ],
  },
];

/* on définit une fonction qui lorsqu'on l'appelle donne qui inverse la valeur de la variable showBranchMenu 
     l'inverse de faux ==> vrai */
