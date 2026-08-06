
<script lang="ts">
    import { createGitgraph } from "@gitgraph/js";
    import { myGitTheme } from "$lib/config/gitTheme";
    import type { CommitInfo } from "$lib/config/GitActionsMenu";

    let { 
    activeView, 
    commits = [] 
    }: { 
    activeView: "split" | "actions" | "tree"; 
    commits: CommitInfo[]; 
    } = $props();
        let gitgraphElement = $state<HTMLDivElement>();


    function renderGitGraph(commitsList: CommitInfo[]) { // on commitInfo pour éviter un conflit avec commits

    if (!gitgraphElement) { // si la référence de div est vide (null / undefined), renvoie true  
        return; // le return permet d'empêcher le reste de la fonction de s'executer si c'est vrai 
    } 


    gitgraphElement.innerHTML = ""; // On néttoie la DOM à chaque fois quela fonction renderGitGraph est appelé pour éviter pour déssiner un nouvel arbre git


    const gitgraph = createGitgraph(gitgraphElement, { template: myGitTheme });

    const branches: Record<string, any> = {}; // on créer un objet vide, Record<> indique qu'il s'agit d'un objet, string indique que la clé sera du string, any indique que la valeur peut etre de n'importe quel type
    branches["main"] = gitgraph.branch("main"); // Reçoit le bleu par défaut

    const reversedCommits = [...commitsList].reverse(); // reverse inverse l'ordre des éléments une liste, on le reverse pour déssiner du bas (commit les plus ancien) vers le haut (commit les plus récents)
    const totalCommits = reversedCommits.length

    reversedCommits.forEach((c, index) => {
        const branchName = (Array.isArray(c.branches) ? c.branches[0] : c.branches) || "main";

        if (!branches[branchName]) {
        branches[branchName] = branches["main"].branch(branchName);
        }

        const isHead = index === totalCommits - 1;

        branches[branchName].commit({
        subject: c.message,
        hash: c.id,
        author: c.author,
        tag: isHead ? "HEAD" : undefined
        });
    });
    }

    $effect(() => {
        renderGitGraph(commits);
    });
</script>

<div 
    class="tree-wrapper"
    class:hidden={activeView === "actions"}  
    class:full-width={activeView === "tree"} 
    bind:this={gitgraphElement} 
></div>


<style>

.tree-wrapper {
    background-color: #2a2a2a;
    border: 2px dashed #666666;
    flex: 1;
    color: #aaaaaa;
    font-family: "Inter", sans-serif;
    font-size: 1.2rem;
    font-weight: bold;
    border-radius: 6px;
    box-sizing: border-box;
    display: grid;
    place-items: center;
    overflow: auto;
    opacity: 1;
    
    transition: flex 0.4s cubic-bezier(0.4, 0, 0.2, 1), opacity 0.25s ease-in-out, padding 0.4s ease;
}

.tree-wrapper :global(svg) {
    padding: 60px; 
}

.tree-wrapper :global(svg circle) { 
    cursor: pointer;
}

.tree-wrapper:active {
    cursor: grabbing;
}

.full-width {
  flex: 999;
}

.hidden {
  flex: 0 !important;
  width: 0 !important;
  padding: 0 !important;
  opacity: 0;
  pointer-events: none;
  border: none !important;
}

</style>