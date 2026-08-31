use std::process::Command;

#[tauri::command]
pub fn get_actif_files(path:String) -> Result <String, String>{
    let output = Command::new("git")
    .args(["status", "--porcelain"])
    .current_dir(&path)
    .output()
    .map_err(|e| e.to_string())?;

    // on déclare son type de retour
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
    else{
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

#[tauri::command]
pub fn get_not_pushed_commits(path:String) -> Result <String, String>{
    let output = Command::new("git")
    .args(["log", "@{u}..HEAD"])
    .current_dir(&path)
    .output()

    .map_err(|e| e.to_string())?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
    else{
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    // scénario dans lequel on teste le comportement dans cas ou un fichié n'est pas suivi
    fn get_actif_files_not_tracked(){
        let temp_dir = tempfile::tempdir().unwrap();
        git2::Repository::init(temp_dir.path()).unwrap();

        // on créer un fichier qui n'a pas été indexé
        std::fs::write(temp_dir.path().join("fichier_exemple.txt"), "contenu").unwrap();

        let temp_dir_string = temp_dir.path().to_string_lossy().into_owned();

        let result = get_actif_files(temp_dir_string);

        assert!(result.is_ok());
        assert!(result.unwrap().contains("?? fichier_exemple.txt"))
    }

    #[test]
    // scénario dans lequel on teste le comportement dans cas ou un fichié est suivi
    fn get_actif_files_files_are_tracked(){
        let temp_dir = tempfile::tempdir().unwrap();
        git2::Repository::init(temp_dir.path()).unwrap();

        let temp_dir_string = temp_dir.path().to_string_lossy().into_owned();
        let result = get_actif_files(temp_dir_string);

        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }


    // scénario dans lequel le commit est déja push donc git log @{u}.HEAD renvoie rien
    #[test]
    fn get_not_pushed_commits_when_commits_are_pushed(){
        
        let temp_dir = tempfile::tempdir().unwrap();
        git2::Repository::init(temp_dir.path()).unwrap();
        std::fs::write(temp_dir.path().join("fichier_exemple.txt"), "contenu").unwrap();

    }
}
