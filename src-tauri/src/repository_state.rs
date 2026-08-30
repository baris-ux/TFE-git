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