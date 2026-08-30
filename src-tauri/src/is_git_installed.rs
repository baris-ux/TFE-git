use std::process::Command;

#[tauri::command]
pub fn verify_if_git_installed() -> bool{
    let output = Command::new("git") // execution du programme git 
    .arg("--version") // on passe en argument à la commande git, --version
    .output(); // renvoie un type Result, 2 états possible OK(valeur) Err(valeur) qui sont tout les deux dans des boites fermé

    match output {
        Ok(res) => res.status.success(),
        Err(_) => false
    }
}

#[cfg(test)]    
mod tests {
    use super::*;

    // Scénario : git est installé sur la machine   
    // on vérfie juste que test_verify_if_git_installed renvoie true 
    #[test]
    fn test_verify_if_git_installed(){
        let result = verify_if_git_installed();
        assert!(result)
    }
}