// eslint.config.js
import { defineConfig } from "eslint/config";
import js from "@eslint/js";
import ts from "typescript-eslint";
import svelte from "eslint-plugin-svelte";
import globals from "globals";

export default defineConfig([
  js.configs.recommended,
  /* c'est un ensemble de règle préconfiguré pour JavaScript ca permet d'éviter d'écrire 100 règle à la main
    ca permet par exemple d'empêcher de réassigner une valeur à une variable déclarée avec const*/
  ...ts.configs.recommended,

  ...svelte.configs["flat/recommended"], // meme principe mais avec du svelte cette fois ci

  {
    files: ["**/*.svelte"],
    languageOptions: {
      globals: {
        ...globals.browser, // on indique avec cette ligne que des mots tels que windows, document ou console sont normal et qu'il ne doitvent pas être
        // marqué comme erreur
        ...globals.node,
        // on indique que le mot process ne doit pas être marqué comme une erreur
      },
      parserOptions: {
        parser: ts.parser,
      },
    },
  },
  {
    files: ["vite.config.js"],
    languageOptions: {
      globals: {
        ...globals.node,
      },
    },
  },
  /* ici pour les fichier en .svelte on va venir parser le code typescript ce qui permettra à 
    notre linter de lire et comprendre le code typescript pour le linter*/

  {
    rules: {
      // pour le moemnt on va partir avec les règle par défaut spécifié par js.configs.recommended et svelte.configs['flat/recommended']
      // mais on peut venir ajouter nos propres règles ici
    },
  },
  {
    ignores: [
      "node_modules/**",
      "dist/**",
      ".svelte-kit/**",
      "src-tauri/**",
      "build/**",
    ],
  },
]);
