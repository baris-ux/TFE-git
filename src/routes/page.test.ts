import { vi, describe, it, expect } from "vitest";
import { goto } from "$app/navigation";
import { render } from "@testing-library/svelte"; // sert à charger et manipuler le composant Svelte dans un faux navigateur pendant le tes
import Page from "./+page.svelte";

// Un mock est une doublure factice d'une dépendance externe
// L'espion (vi.fn()) remplace le vrai mécanisme pour deux choses :

//1. Agir comme un bouclier : empêcher le vrai code externe de s'exécuter et de faire planter le test.
//2. Agir comme une boîte noire : enregistrer tous les appels (paramètres reçus, nombre d'exécutions) pour te permettre de vérifier que ton composant a bien fait son travail.

// vi.mock prend en paramètre ==> le chemin du fichier ou le nom du package importé sous forme de chaîne de caractères
// le 2e paramètre indique l'objet à l'intéreur du colis qu'on va envoyer à la bonne adresse
// le return définit l'objet javascript qui sera livré à $app/navigation

// à l'intérieur de l'objet on met des fonctions espionnes vi.fn(), elle remplace la vrai fonction par une doublure
// cette doublure enregistre tout ce qui lui arrive, elle note si :
// elle à été appelé, cmb de fois elle à été appelé, quel argument lui on été passé,
//
// Par défaut, vi.fn() n'appelle pas la fonction d'origine.

vi.mock("$app/navigation", () => {
  return {
    goto: vi.fn(),
  };
});

vi.mock("@tauri-apps/api/core", () => {
  return {
    invoke: vi.fn(),
  };
});

vi.mock("@tauri-apps/plugin-opener", () => {
  return {
    openUrl: vi.fn(),
  };
});

vi.mock("$app/paths", () => {
  return {
    resolve: (path: string) => path,
  };
});

// la fonction describe permet de regrouper plusieurs tests associé à une même fonctionnalité
// elle prend 2 paramètres, le nom du groupe et la fonction callback
describe("dd", () => {
  it("redirige automatiquement vers /app si le cache est présent", () => {
    localStorage.setItem("git-is-installed", "true");

    render(Page); // render permet de monter le composant svelte page dans un faux navigateur

    expect(goto).toHaveBeenCalledWith("/app"); // notre assertion, permet d'indiquer ce que le code est censé produire pour le scénario donné
  });

  it("Ne pas rediriger si le cache est absent", () => {
    render(Page);

    expect(goto).not.toHaveBeenCalled;
  });
});
