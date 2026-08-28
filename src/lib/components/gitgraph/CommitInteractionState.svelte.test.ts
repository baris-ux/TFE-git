import { beforeEach, describe, expect, it, vi } from "vitest";
import { CommitInteractionState } from "./CommitInteractionState.svelte";
import { invoke } from "@tauri-apps/api/core";

vi.mock("@tauri-apps/api/core", () => {
  return {
    invoke: vi.fn(),
  };
});

describe("openBoxOnCommitClick", () => {
  // les variables state et path sont paratégé entre tout les it(), comme on va avoir un 2e test
  // il faut les recréer avant chaque test

  let state: CommitInteractionState;
  let path = "voiciunchemin";
  it("j'ai cliqué sur un noeud et la comparaison n'est pas active", () => {
    const commit = {
      id: "abc1234",
      author: "baris",
      parents: ["efg567"],
      branches: ["feature/mailbox"],
      is_head: false,
      message: "voici un exemple",
    };
    state.openBoxOnCommitClick(commit, path);

    expect(state.selectedCommit).toEqual(commit);
    expect(state.commitInfoDisplayed).toBe(false);
    expect(state.isBarActive).toBe(true);
  });

  beforeEach(() => {
    state = new CommitInteractionState();
  });
  it("j'ai cliqué sur un noeud avec un hash identique au premier", () => {
    state.isComparaisonActive = true;
    state.firstHash = "hij789";
    const commit = {
      id: "hij789",
      author: "baris",
      parents: ["ejm839"],
      branches: ["feature/house"],
      is_head: false,
      message: "je n'ai pas d'inspiration",
    };

    state.openBoxOnCommitClick(commit, path);

    expect(state.diffError).toBe("Choisis un commit différent du premier.");
  });

  it("j'ai cliqué sur un noeud avec un hash différent au premier", () => {
    state.isComparaisonActive = true;
    state.firstHash = "dpc394";
    const commit = {
      id: "psm304",
      author: "baris",
      parents: ["mdl290"],
      branches: ["feature/billing"],
      is_head: false,
      message: "je n'ai toujours pas d'inspiration",
    };

    state.openBoxOnCommitClick(commit, path);

    expect(state.secondHash).toBe(commit.id);
  });
});

describe("commitComparaison", () => {
  let state = new CommitInteractionState();
  beforeEach(() => {
    state = new CommitInteractionState();
  });
  let path = "voiciunchemin";

  it("j'ai cliqué sur un noeud qui à un hash null", () => {
    state.firstHash = null;
    state.secondHash = "def456";

    state.commitComparaison(path);

    expect(state.diffError).toBe(null);
  });

  it("les hash et le path sont valides et invoke renvoie un résultat", async () => {
    state.firstHash = "abc123";
    state.secondHash = "def456";

    vi.mocked(invoke).mockResolvedValue("ils sont différent ici et ici");

    await state.commitComparaison(path); //

    expect(state.diffResult).toBe("ils sont différent ici et ici");
  });
});
