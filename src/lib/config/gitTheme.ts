import { templateExtend, TemplateName } from "@gitgraph/js";


export const myGitTheme = templateExtend(TemplateName.Metro, { // le template par défaut est metro, on a également blackarrow templateExtend() permet de créer notre propre template
    colors: [
        "#3b82f6", // bleu branche main
        "#10b981", // 2ème branche -> VERT
        "#f59e0b", // 3ème branche -> ORANGE
        "#ec4899", // 4ème branche -> ROSE
        "#8b5cf6"  // 5ème branche -> VIOLET
    ]
});

