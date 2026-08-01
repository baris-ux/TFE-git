# Tauri + SvelteKit + TypeScript

This template should help get you started developing with Tauri, SvelteKit and TypeScript in Vite.

## Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) + [Svelte](https://marketplace.visualstudio.com/items?itemName=svelte.svelte-vscode) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).


## 🛠️ explication du package.json

* **`lint": "eslint .`** 
    Il permet l'execution de la commande pnpm lint qui permet d'aller chercher tous les dans tous fichiers du projet (sauf ceux ignorés dans `eslint.config.js`) les mauvaises pratiques et maintenir la qualité du projet.


* **`@tauri-apps/plugin-dialog .`**
* C'est un Package JS qui permet d'ouvrir l'explorateur de fichiers. Il permet de sélectionner un dossier et de renvoyer son chemin. Souvenez-vous, Node est un runtime qui a accès à notre disque et qui n'est pas sandboxé. Or, dans notre projet, on ne possède pas Node.js. Le runtime JavaScript utilisé dans ce projet est le composant Webview, qui lui est sandboxé. C'est pourquoi ce package contient du code JavaScript qui permet de communiquer via IPC avec Rust, qui lui a un accès au disque.

