use git2::Repository;
use serde::Serialize; // import qui permet d'utiliser #[derive(Serialize)]
use std::process::Command;
use std::fs; // pour créer un folder

use std::collections::HashMap;

#[derive(Serialize)]
pub struct CommitInfo {
    pub id: String,
    pub message: String,
    pub author: String,
    pub parents: Vec<String>,
    pub branches: Vec<String>, //
}

#[tauri::command]
fn setup_exercise_repo(
    setup_commands : Vec<Vec<String>>, // array d'array contenant string
    id: String
) -> Result<String, String> { // vec pour vector, un tableau dynamique qui contient des chaine de caractère

    let mut desktop_path = dirs::desktop_dir() // rappel : mut est spécifié pour mutable
    // dirs::desktop_dir() est une fonction qui renvoie une boite Option<PathBuf> elle même contenant  Some(chemin) si le bureau existe, None si pas trouvé ==> donc on a Some(chemin) ou None 
        .ok_or_else(|| "Impossible de trouver le dossier Bureau sur ce système.".to_string())?; // .ok_or_else transforme la boite Option en boite result (Ok ou Err) ==> donc on a Ok(chemin) ou Err(chemin) ==> la boite Some est détruite
                                                                                                // ? ==> est détruite si  c'est un Ok() dedans, si Err(chemin) dedans et renvoie l'erreur

    desktop_path.push(&id);

    fs::create_dir_all(&desktop_path).map_err(|e| e.to_string())?;
    
    // permet de créer un folder avec le nom de l'exo, le ?
    // Si résultat Ok() il extrait la valeur contenu à l'intérieur 
    // si il échoue il fait un Err() qui doit être déballé et convertit en string pourquoi ? parce qu'on a dit dans Result<String, String> que en cas de réussite on doit avoir un string et pareil en cas d'echec

    // l'idée ici est qu'on va venir boucler sur setupCommands car il s'agit d'une liste de commande git pour venir les executer un à un
    // 

    for cmd in setup_commands { // on parcour notre array d'array, cmd étant un array

        let program = &cmd[0]; // à l'indice 0 on trouvera le mot git
        let args = &cmd[1..]; // à partir d' lindice 1 (donc les argument comme commit, push, checkout ...) jusqu'au dernier élément de la liste

        let output = Command::new(program) // execution du programme, ici git 
        .args(args)
        .current_dir(&desktop_path)
        .output()
        .map_err(|e| format!("Impossible de lancer '{}': {}", program, e))?; // on met program, qui est un string

        if !output.status.success() { // si le output n'a pas pour status succès renvoie un vrai
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Erreur lors de l'exécution de '{}' : {}", program, stderr));
        }
    }
    Ok(desktop_path.to_string_lossy().into_owned())
}


#[tauri::command]
fn verify_tutorial_step(tutorial_id: String, step_index: usize) -> Result<bool, String> {
    match tutorial_id.as_str() {     // tutorial id est un string qu'on convertit en &str, différence entre string et &str est que &str pointe 
        "exo-1" => match step_index { // si tutorial_id  corresspond à exo-1
            0 => { 
                if let Some(desktop) = dirs::desktop_dir() { // j'ai utilisé le package dirs, il renvoie Some(dossier) ou none 
                    let dossier_test = desktop.join("test");
                    Ok(dossier_test.exists() && dossier_test.is_dir())
                } else {
                    Ok(false)
                }
            }
            _ => Ok(false),
        },
        _ => Err("Tutoriel inconnu".into()),
    }
}

#[tauri::command]
fn verify_if_git_installed() -> bool{
    let output = Command::new("git") // execution du programme git 
    .arg("--version") // on passe en argument à la commande git, --version
    .output(); // renvoie un type Result, 2 états possible OK(valeur) Err(valeur) qui sont tout les deux dans des boites fermé

    match output { // output permet d'ouvrir cette "boite"
        Ok(res) => res.status.success(),
        Err(_) => false
    }
}

#[tauri::command]
async fn if_git_repository(path : String) -> bool {
    match Repository::open(&path){ // on utilise & pour indiquer qu'on passe une référence au lieu de le copier, on récupère la valeur du path au lieu de le copier
        Ok(_) => { 
            println!("yes c'est bien ca");
            true
        }
        Err(_) => {
            println!("nope tu t'es trompé");
            false
        }
    }
}

#[tauri::command]
fn get_git(path: String) -> Result<Vec<CommitInfo>, String> {
    let repo = Repository::open(&path).map_err(|e| e.to_string())?;

    /* Repository::open(&path) tente d'ouvrir le dossier Git dans le chemin fournit, le &path permet de récupérer le path au lieu de créer un doublon du path fournit 
    
    .map_err(|e| e.to_string()) vient permettre de à ce que Repository::open(&path) l'erreur (sous forme de structure git2::Error) 
    envoyé par la fonction repository::open(&_path) soit convertit en string 
    
    le ? à la fin permet à ce que la variable repo se voit attribué la valeur du dépot uniquement si 
    Repository::open(&path) a comme résultat git2::Ok, en cas de git2::Error pas de valeur attribué*/

    let mut branch_tips: Vec<(String, git2::Oid)> = Vec::new();

    // Vec est notre array, chaque element sera un tuple avec deux valeur un string (nom de la bracche) et git2:Oid (le hash du commit parent)

    /* -------------------------- IA ----------------------------- */
    if let Ok(branches) = repo.branches(Some(git2::BranchType::Local)) {
        for b in branches.flatten() {
            let (branch, _) = b;
            if let (Ok(Some(nom)), Some(target)) = (branch.name(), branch.get().target()) {
                branch_tips.push((nom.to_string(), target));
            }
        }
    }

    branch_tips.sort_by_key(|(nom, _)| if nom == "main" { 0 } else { 1 }); // on trie le array en fonction, on vient détruire la tuple pour en extraire la valeur nom 
    // if nom est main il retourne 0 si autre autre que main alors retourne 1

    let mut owner: HashMap<String, String> = HashMap::new(); 
    // pour rappelle un hashmap est un structure qui contient des information sous la forme clé : valeur
    // contrairement à structure on vient uniquement définir le type des clé (ici string) et leur type de valeur (string auss)

    for (branch_name, tip_oid) in &branch_tips {
        let mut branch_revwalk = repo.revwalk().map_err(|e| e.to_string())?;
        branch_revwalk.push(*tip_oid).map_err(|e| e.to_string())?;
        branch_revwalk.simplify_first_parent().map_err(|e| e.to_string())?; // ne suit que le 1er parent, pour ne pas "fuiter" dans l'historique fusionné d'un merge

        for id in branch_revwalk {
            let oid = id.map_err(|e| e.to_string())?;
            let commit_id_str = oid.to_string();
            if owner.contains_key(&commit_id_str) { 
                break;
            }

            owner.insert(commit_id_str, branch_name.clone());
        }
    }

    /* -------------------------- IA ----------------------------- */

    let mut revwalk = repo.revwalk().map_err(|e| e.to_string())?;
    revwalk.push_glob("refs/heads/*").map_err(|e| e.to_string())?;
    revwalk.set_sorting(git2::Sort::TOPOLOGICAL | git2::Sort::TIME).map_err(|e| e.to_string())?; // IA

    /* repo.revwalk() instancie l'outil qui va permettre de parcourir tout les messages de commit dans notre projet 
    
    on vient mettre en string la structure git2::error en cas d'erreur car les fichiers en interne .git peut être corrompu
    contrairmeent à ce qu'on peut croire let mut revwalk ne va pas se voir attribué les commit du projet, let mut revwalk se voit attribué l'itérateur revwalk
    
    En cas de succès uniquement, en cas d'erreur il ne recevra pas de valeur car on a le ? */

    // revwalk.push_head().map_err(|e| e.to_string())?; on indique à l'itérateur revwalk de commencer son parcour à partir de HEAD

    let mut commits = Vec::new(); // on créer une list  vide mutable car on va ajouter des items dedans

    for id in revwalk { // on remonte vers le commit le plus anciens
        let oid = id.map_err(|e| e.to_string())?; // oid (object identifier), on attribue le hash en cas de non erreur 
        let commit = repo.find_commit(oid).map_err(|e| e.to_string())?;
        // commilt on attribue à commit la structure rust (objet) du commit qu'on cherche sur base du hash, cette structure rust contient le message, l'auteur, la date etc ... du commit 

        let commit_id_str = oid.to_string();
        let parents = commit.parent_ids().map(|p| p.to_string()).collect();

        /* ------------ IA-------------*/
        let mes_branches: Vec<String> = owner
            .get(&commit_id_str)
            .map(|b| vec![b.clone()])
            .unwrap_or_default();
        /* ------------ IA-------------*/

        commits.push(CommitInfo {
            id: oid.to_string(),
            message: commit.summary().ok().flatten().unwrap_or("").to_string(),

            /*commit notre structure rust 

            .summary() est une méthode de la librairie git2 qui sert à extraire 
            la première ligne d'un commit le résultat obtenu est protégé dans DEUX boite fermé Result et Option
            
            .ok() permet de continuer l'execution d'une fonction même en cas d'erreur, 
            contrairement à ? qui arrête l'execution de la fonction en cas d'erreur pour renvoyer l'erreur 

            .ok() permet de convertir la boite Result en boite Option
            on a deux boite Option 

            .flatten() permet de fusionner ces deux boites Options en former une

            .unwrap_or() permet d'ouvrir la dernière boite option 

            */
            author: commit.author().name().unwrap_or("Inconnu").to_string(),
            parents,
            branches: mes_branches,
        });

        // on vient push dans la liste vide commits 
    }

    Ok(commits)
}

// ---------------------------------------------------------- Tests unitaires ----------------------------------------------------------

#[cfg(test)]
mod tests {

    use super::*;
    // Scénario : dépot avec un seul commit racine sur une seule branche
    // Vérifie que get_git extrait correctement id/message/author et que parents est vide
    #[test]
    fn test_get_git_single_commit() {
        let temp_dir = tempfile::tempdir().unwrap(); // tempdir renvoie un Result il faut le déballer pour récupérer la valeur
        let repo = git2::Repository::init(temp_dir.path()).unwrap(); // on initalise le dépot git 
        let signature = git2::Signature::now("Alice", "alice@test.com").unwrap(); 

        
        let mut index = repo.index().unwrap(); 
        let tree_oid = index.write_tree().unwrap();
        let tree = repo.find_tree(tree_oid).unwrap();

        /* -----------------------------------  IA ----------------------- ----------------------- */

        let commit_oid = repo.commit(
            Some("HEAD"),
            &signature,
            &signature,
            "Initial commit",
            &tree,
            &[],
        ).unwrap();

        let result = get_git(temp_dir.path().to_string_lossy().into_owned());

        assert!(result.is_ok());
        let commits = result.unwrap();
        assert_eq!(commits.len(), 1);
        assert_eq!(commits[0].id, commit_oid.to_string());
        assert_eq!(commits[0].message, "Initial commit");
        assert_eq!(commits[0].author, "Alice");
        assert!(commits[0].parents.is_empty());
        /* -----------------------------------  IA ----------------------- ----------------------- */
    }

    // Scénario : le path en paramètre n'est pas cel d'un dépot git valide, le .git/ n'y est pas
    // Vérfie que get_git renvoie bien une erreur 
    #[test]
    fn get_git_invalid_path(){
        let temp_dir = tempfile::tempdir().unwrap(); // on vient de nouveau créer un dossier temporaire
        let result = get_git(temp_dir.path().to_string_lossy().into_owned()); // temp_dir.path renvoi un &Path alors que get_git attend un String
        // .to_string_lossy() renvoie un Cow<str> 
        // .into_owned() convertit en string peut importe si c'est Cow::Borrowed ou Cow::Owned

        assert!(result.is_err());
    }

    #[test]
    fn test_get_git_diverging_branches(){ 
        let temp_dir = tempfile::tempdir().unwrap();
        let repo = git2::Repository::init(temp_dir.path()).unwrap();
        let signature = git2::Signature::now("Alice", "alice@test.com").unwrap(); 

        let mut index = repo.index().unwrap(); // index est un dictionnaire sous la forme clé valeur, chaque entrée correspond à
        // au path du fichié (clé) et le hash du fichier (valeur)
        let tree_oid = index.write_tree().unwrap(); // .write_tree prend une copie du dictionnaire index et le transforme en tree
        // contrairmeent au dictionnaire index le contenu de ce dictionaire est immuable
        let tree = repo.find_tree(tree_oid).unwrap();

         /* -----------------------------------  IA ----------------------- ----------------------- */

        let base_commit_oid = repo.commit(
            Some("HEAD"),
            &signature,
            &signature,
            "Base commit",
            &tree,
            &[],
        ).unwrap();


        let base_commit = repo.find_commit(base_commit_oid).unwrap();
        repo.branch("feature", &base_commit, false).unwrap();

        let commit_on_main_oid = repo.commit(
            Some("HEAD"),
            &signature,
            &signature,
            "Commit on main",
            &tree,
            &[&base_commit],
        ).unwrap();

        let commit_on_feature_oid = repo.commit(
            Some("refs/heads/feature"),
            &signature,
            &signature,
            "Commit on feature",
            &tree,
            &[&base_commit],
        ).unwrap();

        let result = get_git(temp_dir.path().to_string_lossy().into_owned());
        assert!(result.is_ok());
        let commits = result.unwrap();

        assert_eq!(commits.len(), 3);

        let base = commits.iter().find(|c| c.id == base_commit_oid.to_string()).unwrap();
        let on_main = commits.iter().find(|c| c.id == commit_on_main_oid.to_string()).unwrap();
        let on_feature = commits.iter().find(|c| c.id == commit_on_feature_oid.to_string()).unwrap();

        assert!(base.parents.is_empty());
        assert_eq!(on_main.parents, vec![base_commit_oid.to_string()]);
        assert_eq!(on_feature.parents, vec![base_commit_oid.to_string()]);

         /* -----------------------------------  IA ----------------------- ----------------------- */
    }
}

// #[cfg_attr(mobile, tauri::mobile_entry_point)] ==> si on compile le projet sur android ou IOS il génère le code necessaire pour le fonctionne
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init()) // ca permet d'ouvrir un browser dans l'os (plus pratique que si on ouvre dans le webview)
        .plugin(tauri_plugin_pty::init()) // ajout du plugin pty permettant de demander à l'os de créer un pseudo terminal
        .plugin(tauri_plugin_dialog::init()) // ajout du plugin dialog permettant d'ouvrir l'explorateur de fichiers de l'os
        .invoke_handler(tauri::generate_handler![if_git_repository, get_git, verify_tutorial_step, verify_if_git_installed, setup_exercise_repo])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}



