use git2::Repository;
use serde::Serialize; // import qui permet d'utiliser #[derive(Serialize)]

#[derive(Serialize)]
pub struct CommitInfo {
    pub id: String,
    pub message: String,
    pub author: String,
    pub parents: Vec<String>,
    pub branches: Vec<String>,
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
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
    envoyé par la fonction repository::open(&path) soit convertit en string 
    
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
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_pty::init()) // ajout du plugin pty permettant de demander à l'os de créer un pseudo terminal
        .plugin(tauri_plugin_dialog::init()) // ajout du plugin dialog permettant d'ouvrir l'explorateur de fichiers de l'os
        .invoke_handler(tauri::generate_handler![greet, if_git_repository, get_git])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}



