use git2::Repository;
use serde::Serialize; // import qui permet d'utiliser #[derive(Serialize)]
use std::process::Command;
use std::fs; // pour créer un folder

#[derive(Serialize)]
pub struct CommitInfo {
    pub id: String,
    pub message: String,
    pub author: String,
    pub parents: Vec<String>,
    pub branches: Vec<String>,
}

#[tauri::command]
fn setup_exercise_repo(
    setup_commands : Vec<String>, // ajout du underscore _ pour éviter le warning de non utilisation 
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

    for cmd in setup_commands {
        let parts: Vec<&str> = cmd.split_whitespace().collect(); // la commande git on va le split pour couper les espaces avec la fonction split_whitespace() 
                                                                // puis on utilise la fonction .collect qui va mettre les élément dans un liste 
                                                                // par exemple git commit -m "essais" ==> ["git", "commit", "-m", "essais"]

        let program = parts[0]; // à l'indice 0 on trouvera le mot git
        let args = &parts[1..]; // à partir d' lindice 1 (donc les argument comme commit, push, checkout ...) jusqu'au dernier élément de la liste

        let output = Command::new(program) // execution du programme, ici git 
        .args(args)
        .current_dir(&desktop_path)
        .output()
        .map_err(|e| format!("Impossible de lancer '{}': {}", cmd, e))?;

        if !output.status.success() { // si le output n'a pas pour status succès renvoie un vrai
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Erreur lors de l'exécution de '{}' : {}", cmd, stderr));
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
    let output = Command::new("git")
    .arg("--version")
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
fn get_git(path : String) -> Result<Vec<CommitInfo>, String> {
    let repo = Repository::open(&path).map_err(|e| e.to_string())?; 
    
    /* Repository::open(&path) tente d'ouvrir le dossier Git dans le chemin fournit, le &path permet de récupérer le path au lieu de créer un doublon du path fournit 
    
    .map_err(|e| e.to_string()) vient permettre de à ce que Repository::open(&path) l'erreur (sous forme de structure git2::Error) 
    envoyé par la fonction repository::open(&_path) soit convertit en string 
    
    le ? à la fin permet à ce que la variable repo se voit attribué la valeur du dépot uniquement si 
    Repository::open(&path) a comme résultat git2::Ok, en cas de git2::Error pas de valeur attribué*/


    let mut revwalk = repo.revwalk().map_err(|e| e.to_string())?;
    revwalk.push_head().map_err(|e| e.to_string())?; 

    /* repo.revwalk() instancie l'outil qui va permettre de parcourir tout les messages de commit dans notre projet 
    
    on vient mettre en string la structure git2::error en cas d'erreur car les fichiers en interne .git peut être corrompu
    contrairmeent à ce qu'on peut croire let mut revwalk ne va pas se voir attribué les commit du projet, let mut revwalk se voit attribué l'itérateur revwalk
    
    En cas de succès uniquement, en cas d'erreur il ne recevra pas de valeur car on a le ? */


    // revwalk.push_head().map_err(|e| e.to_string())?; on indique à l'itérateur revwalk de commencer son parcour à partir de HEAD


    let mut commits = Vec::new(); // on créer une list  vide mutable car on va ajouter des items dedans

    for id in revwalk {

        let oid = id.map_err(|e| e.to_string())?; // oid (object identifier), on attribue le hash en cas de non erreur 
        let commit = repo.find_commit(oid).map_err(|e| e.to_string())?; 
        // commilt on attribue à commit la structure rust (objet) du commit qu'on cherche sur base du hash, cette structure rust contient le message, l'auteur, la date etc ... du commit 

        let parents = commit.parent_ids().map(|p| p.to_string()).collect(); // on 

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
            branches: vec![],
        });

        // on vient push dans la liste vide commits 
    }

    Ok(commits)
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



