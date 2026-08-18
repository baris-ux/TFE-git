# GitLearn application de bureau cross-platform

Bienvenue sur le README de mon application de bureau GitLearn que j'ai faite dans le cadre de mon travail de fin d'études.
Il s'agit d'un outil permettant de faciliter l'apprentissage de Git. À l'intérieur, vous trouverez les fonctionnalités suivantes :

## Fonctionnalités : 

- un terminal qui est relié au shell de votre machine dans lequel vous pourrez entrer vos commandes Git
- une représentation visuelle de l'arbre Git où vous pourrez voir les branches, les nœuds
- interaction avec l'arbre Git pour afficher les détails de commit et la différence avec un autre commit
- un menu d'actions guidées qui génère la commande dans le terminal
- une fiche d'information pour chaque commande dans le menu d'actions guidées
- un 1er tutoriel indiquant comment utiliser Git
- un 1er exercice permettant de pratiquer Git

## Technologies utilisées

Pour réaliser mon application de bureau, j'ai utilisé Tauri comme framework ainsi que Svelte pour le frontend UI.


## Installation

Pour installer l'application, rendez-vous dans la section
[**Releases du dépôt**](https://github.com/baris-ux/TFE-git/releases).

### Dernière version stable

Elle porte l'étiquette `Latest`. À noter que la version actuellement marquée
`Latest` (la version v1.0.2) constitue une exception : par erreur de configuration, elle n'a pas été
publiée en préversion et ne comporte pas un identifiant de préversion alors qu'elle n'était pas encore stable. Les prochaines
versions seront correctement étiquetées.

<img width="961" height="155" alt="image" src="https://github.com/user-attachments/assets/f3f720ae-0741-4a1c-9711-11f02d83c6c1" />

### Dernière version en date

Vous pouvez également récupérer la version la plus récente, tout en haut de la
page. Celle-ci porte l'étiquette `Pre-release` : le développement est en cours
et certaines fonctionnalités peuvent être incomplètes.

<img width="961" height="155" alt="image" src="https://github.com/user-attachments/assets/9e5d116e-fcfa-47e9-af11-72d97e7d991e" />

### Fichier à télécharger

Vous trouverez le fichier exécutable correspondant à votre système d'exploitation.

Vous y trouverez le fichier exécutable correspondant à votre système d'exploitation :
- **Linux :** `.deb` ou `.AppImage`
- **Windows :** `.exe`
- **macOS :** `.dmg`



### Installation sur Ubuntu

Téléchargez le fichier `.deb`, puis depuis le dossier où il se trouve :

```bash
cd ~/Téléchargements
sudo apt install ./git-learn_1.1.0-3_amd64.deb
```

Le `./` est indispensable : sans lui, `apt` cherche le paquet dans ses dépôts au
lieu de lire le fichier local. N'extrayez pas le `.deb` avec un gestionnaire
d'archives, il doit être installé tel quel.

Lancez ensuite l'application depuis le menu des applications, ou en terminal :

```bash
git-learn
```

Pour la désinstaller :

```bash
sudo apt remove git-learn
```

### Alternative sans installation (Linux)

Le format `.AppImage` ne nécessite aucune installation ni droits administrateur :

```bash
chmod +x git-learn_1.1.0-3_amd64.AppImage
./git-learn_1.1.0-3_amd64.AppImage
```

