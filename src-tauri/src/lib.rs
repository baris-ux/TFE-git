// vous trouverez ici mon code rust, c'est un langage que j'ai eu beaucoup de mal à comprendre
// pour faciliter votre compréhension j'ai mis des commentaire un peu partout en esperent que cela aidera à la compréhension
// si vous devez retenir une seule chose en rust  la philosophie est que 
// vous devez TOUT mais alors absolument TOUT convetir dans le format attendu pour que le type correspond !!!


use serde::Serialize; // import qui permet d'utiliser #[derive(Serialize)]

mod is_git_installed;
mod git_repository;
mod tuto_exercice_modules;

#[derive(Serialize)]
pub struct CommitInfo {
    pub id: String,
    pub message: String,
    pub author: String,
    pub parents: Vec<String>,
    pub branches: Vec<String>, 
    pub is_head: bool,
}


#[tauri::command]
fn compare_commit(path: &str, old_commit: &str, new_commit: &str) -> Result<String, String>{
    let output = std::process::Command::new("git")
        .current_dir(path)
        .args(["diff", old_commit, new_commit])
        .output() // renvoie un Result
        .map_err(|err| format!("Impossible d'exécuter Git : {err}"))?; // .map_err transforme l'erreur si resultat est un Err

        if !output.status.success(){// le statut de fin du processus git .sucess() ==> si 0 alors true, si false != 0 
            return Err(String::from_utf8_lossy(&output.stderr).to_string()); 
            // après executiond de git utput.stderr contient son message d’erreur.
            // par exemple si le hash du commit est mauvais git renvoie fatal: bad object abc123
            // mais rust recoit la réponse sous forme d'octets

            // from_utf8_lossy() transforme &[u8] en Cow<'_, str>
        } 

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
}


// ---------------------------------------------------------- Tests unitaires ----------------------------------------------------------

#[cfg(test)]
mod tests {

    use super::*;


    // Scénario : si un des commit passé en paramètre (le hash) est invalide
    // on vérifie si un des hash est invalide
    #[test]
    fn test_compare_commit_returns_error_when_old_commit_is_invalid() {
        let temp_dir = tempfile::tempdir().unwrap();
        let repo = git2::Repository::init(temp_dir.path()).unwrap();
        let signature = git2::Signature::now("Alice", "alice@test.com").unwrap();

        let mut index = repo.index().unwrap();
        let tree_oid = index.write_tree().unwrap();
        let tree = repo.find_tree(tree_oid).unwrap();

        let valid_commit = repo
            .commit(
                Some("HEAD"),
                &signature,
                &signature,
                "Commit valide",
                &tree,
                &[],
            )
            .unwrap();

        let result = compare_commit(
            temp_dir.path().to_string_lossy().as_ref(),
            "0000000000000000000000000000000000000000",
            &valid_commit.to_string(),
        );

        assert!(result.is_err());
    }
}

// #[cfg_attr(mobile, tauri::mobile_entry_point)] ==> si on compile le projet sur android ou IOS il génère le code necessaire pour le fonctionne
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init()) // ca permet d'ouvrir un browser dans l'os (plus pratique que si on ouvre dans le webview)
        .plugin(tauri_plugin_pty::init()) // ajout du plugin pty permettant de demander à l'os de créer un pseudo terminal
        .plugin(tauri_plugin_dialog::init()) // ajout du plugin dialog permettant d'ouvrir l'explorateur de fichiers de l'os
        .invoke_handler(tauri::generate_handler![
            compare_commit, 
            git_repository::if_git_repository, 
            git_repository::get_git,  
            is_git_installed::verify_if_git_installed, 
            tuto_exercice_modules::verify_tutorial_step,
            tuto_exercice_modules::setup_exercise_repo
            ]
        )
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}



