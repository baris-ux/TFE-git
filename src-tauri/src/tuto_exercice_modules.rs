use std::process::Command;
use std::fs; // pour créer un folder


#[tauri::command]
pub fn setup_exercise_repo(
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
pub fn verify_tutorial_step(tutorial_id: String, step_index: usize) -> Result<bool, String> {
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
