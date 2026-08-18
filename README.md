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

### Installation sur Windows

Téléchargez le fichier `.exe`, puis double-cliquez dessus depuis votre dossier
de téléchargement.

#### Avertissement SmartScreen

Windows SmartScreen bloquera très probablement l'exécution du fichier. Cela ne
signifie pas que l'application contient un logiciel malveillant : le blocage est
dû à l'absence de signature numérique. Signer une application nécessite un
certificat payant délivré par une autorité de certification, ce qui n'a pas été
mis en place dans le cadre de ce projet.

Pour continuer, cliquez sur **Informations complémentaires** :

<img width="652" alt="Avertissement SmartScreen" src="https://github.com/user-attachments/assets/ca0f7a62-13f3-4b31-a494-fbdcaf759769" />

Puis sur **Exécuter quand même** :

<img width="661" alt="Bouton Exécuter quand même" src="https://github.com/user-attachments/assets/38864dd6-5887-4734-b68c-802a51e6e31d" />

#### Assistant d'installation

L'assistant classique s'ouvre : choisissez le dossier d'installation, puis
laissez-vous guider jusqu'à l'écran final.

<details>
  
<summary>Voir les captures d'écran de l'assistant</summary>

L'assistant s'ouvre. Cliquez sur **Next** :

<img width="581" alt="Écran d'accueil de l'assistant" src="https://github.com/user-attachments/assets/562158af-179f-42e0-9c60-6b704e10e3e8" />

Choisissez le dossier d'installation, puis cliquez de nouveau sur **Next** :

<img width="581" alt="Choix du dossier d'installation" src="https://github.com/user-attachments/assets/16abad57-39fb-4b88-8b88-5d112db4d8a8" />

L'installation se lance. Une fois terminée, cliquez sur **Next** :

<img width="581" alt="Installation terminée" src="https://github.com/user-attachments/assets/2f7c023a-c514-46bd-9dce-774ee73da132" />

Cliquez enfin sur **Finish**. Vous pouvez laisser les cases cochées pour créer un
raccourci sur le bureau et lancer git-learn immédiatement :

<img width="581" height="477" alt="Capture d&#39;écran 2026-08-18 190347" src="https://github.com/user-attachments/assets/15de6f4e-42df-4fca-952b-eb500983b9ad" />

</details>

### Installation sur macOS

> Cette procédure n'a pas pu être testée : le développement s'est déroulé sous
> Linux et Windows, sans accès à une machine macOS. Les binaires sont générés
> automatiquement par l'intégration continue, mais leur installation n'a pas été
> validée manuellement.

Deux fichiers `.dmg` sont proposés selon votre processeur :

- **Apple Silicon** (M1, M2, M3…) : le fichier `aarch64`
- **Intel** : le fichier `x64`

Ouvrez le `.dmg`, puis glissez l'application dans le dossier **Applications**.

#### Avertissement Gatekeeper

L'application n'étant ni signée ni notarisée par Apple, macOS refusera de
l'ouvrir au premier lancement. Faites un **clic droit** sur l'application, puis
choisissez **Ouvrir** dans le menu contextuel, et confirmez.

Si macOS indique que l'application est « endommagée », la mise en quarantaine
peut être retirée en terminal :

```bash
xattr -d com.apple.quarantine /Applications/git-learn.app
```

> [!WARNING]
> Cette commande retire la vérification de sécurité de macOS **pour cette
> application uniquement**. Les autres logiciels restent protégés par Gatekeeper.
> Ne l'utilisez que pour des logiciels dont vous connaissez la provenance.
