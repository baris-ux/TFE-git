use crate::CommitInfo;
use git2::Repository;
use std::collections::HashMap;

#[tauri::command]
pub async fn if_git_repository(path : String) -> bool {
    match Repository::open(&path){ // on utilise & pour indiquer qu'on passe une référence au lieu de le copier, on récupère la valeur du path au lieu de le copier
        Ok(_) => { 
            true
        }
        Err(_) => {
            false
        }
    }
}

#[tauri::command]
pub fn get_git(path: String) -> Result<Vec<CommitInfo>, String> {
    let repo = Repository::open(&path).map_err(|e| e.to_string())?;

    /* Repository::open(&path) tente d'ouvrir le dossier Git dans le chemin fournit, le &path permet de récupérer le path au lieu de créer un doublon du path fournit 
    
    .map_err(|e| e.to_string()) vient permettre de à ce que Repository::open(&path) l'erreur (sous forme de structure git2::Error) 
    envoyé par la fonction repository::open(&_path) soit convertit en string 
    
    le ? à la fin permet à ce que la variable repo se voit attribué la valeur du dépot uniquement si 
    Repository::open(&path) a comme résultat git2::Ok, en cas de git2::Error pas de valeur attribué*/

    let mut branch_tips: Vec<(String, git2::Oid)> = Vec::new();
    // Vec est notre array, chaque element sera un tuple avec deux valeur un string (nom de la bracche) 
    // et git2:Oid (le hash du commit parent)

    if let Ok(branches) = repo.branches(None) { 

        // .branches() c'est la méthode de git2 permettant de fouille dans .git/refs/heads/ pour trouver les branches existantes
        // le dossier .git/refs possède le sous dossier heads/ et remotes/
        // la méthode .branches() attend un seul paramètre de type Option<BrancheType> 

        for b in branches.flatten() {
            let (branch, _) = b;
            if let (Ok(Some(nom)), Some(target)) = (branch.name(), branch.get().target()) {
                branch_tips.push((nom.to_string(), target));
            }
        }
    }

    branch_tips.sort_by_key(|(nom, _)| if nom == "main" { 0 } else { 1 }); 
    // on trie le array en fonction, on vient détruire la tuple pour en extraire la valeur nom 
    // if nom est main il retourne 0 si autre autre que main alors retourne 1
    // la branche main sera prioritaire lors va parcourir avec revwalk (score : 0)
    // les autre branches seront parcouru par le revwalk selon l'ordr edans lequel ils sont trouvé dans le dossier .git/refs/

    let mut owner: HashMap<String, Vec<String>> = HashMap::new(); 
    // pour rappelle un hashmap est un structure qui contient des information sous la forme clé : valeur
    // contrairement à structure on vient uniquement définir le type des clé (ici string) et leur type de valeur (array de string)

    let head_commit_id = repo
    .head() 
    .ok()
    .and_then(|r| r.peel_to_commit().ok())
    .map(|c| c.id());

    // l'idée ici est qu'on lit le .git/HEAD, le fichier contient un text comme ref: refs/heads/main il indique le chemin vers un second fichier qui lui contient le hash du commit
    // la bonne nouvelle c'est que la méthode .head() de git::2 permet déja de faire tout ca !


    for (branch_name, tip_oid) in &branch_tips {
        let mut branch_revwalk = repo.revwalk().map_err(|e| e.to_string())?;
        branch_revwalk.push(*tip_oid).map_err(|e| e.to_string())?;
        branch_revwalk.simplify_first_parent().map_err(|e| e.to_string())?; 
        // ne suit que le 1er parent, pour ne pas "fuiter" dans l'historique fusionné d'un merge

        for id in branch_revwalk {
            let oid = id.map_err(|e| e.to_string())?;
            let commit_id_str = oid.to_string();

            let list = owner.entry(commit_id_str.clone()).or_default();
            if !list.contains(branch_name) {
                list.push(branch_name.clone());
            }
        }
    }

    let mut revwalk = repo.revwalk().map_err(|e| e.to_string())?;

    revwalk.push_glob("refs/heads/*").map_err(|e| e.to_string())?;
    revwalk.push_glob("refs/remotes/*").map_err(|e| e.to_string())?;

    revwalk.set_sorting(git2::Sort::TOPOLOGICAL | git2::Sort::TIME).map_err(|e| e.to_string())?; 

    /* repo.revwalk() instancie l'outil qui va permettre de parcourir tout les messages de commit dans notre projet 
    
    on vient mettre en string la structure git2::error en cas d'erreur car les fichiers en interne .git peut être corrompu
    contrairmeent à ce qu'on peut croire let mut revwalk ne va pas se voir attribué les commit du projet, let mut revwalk se voit attribué l'itérateur revwalk
    
    En cas de succès uniquement, en cas d'erreur il ne recevra pas de valeur car on a le ? */

    let mut commits = Vec::new(); // on créer une list  vide mutable car on va ajouter des items dedans


    for id in revwalk { // on remonte vers le commit le plus anciens
        let oid = id.map_err(|e| e.to_string())?; // oid (object identifier), on attribue le hash en cas de non erreur 
        let commit = repo.find_commit(oid).map_err(|e| e.to_string())?;
        // commilt on attribue à commit la structure rust (objet) du commit qu'on cherche sur base du hash, cette structure rust contient le message, l'auteur, la date etc ... du commit 

        let is_head = Some(oid) == head_commit_id; // on déclare une variable is_head
        // on emballe oid dans une boite Option  pour qu'il soit du même type que head_commit_id
        // is_head est automatiquement mit à true si les valeur sont les mêmes

        let commit_id_str = oid.to_string();
        let parents = commit.parent_ids().map(|p| p.to_string()).collect();

        let mes_branches: Vec<String> = owner
            .get(&commit_id_str)
            //.map(|b| vec![b.clone()])
            .cloned()
            .unwrap_or_default();

        commits.push(CommitInfo {
            id: oid.to_string(),
            message: commit.summary().ok().flatten().unwrap_or("").to_string(),
            //message: commit.summary().unwrap_or("").to_string(),

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
            is_head,
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

    }


    // Scénario : le dossier existe mais il ne s'agit pas d'un dossier git 
    // Vérifie que le dossier temporaire passé en paramètre ne contient pas un .git
    #[tokio::test] // les fonction décalré avec un async necessite l'utilisation de tokio, 
    async fn test_if_git_repository(){
        let temp_dir = tempfile::tempdir().unwrap(); // tempfile::tempdir() créer un dossier temporaire avce un nom aléatoire
        let result = if_git_repository(
            temp_dir.path().to_string_lossy().into_owned() // temp_dir.path() renvoie le chemin du dossier temporaire sous forme &Path
            // to_string_lossy() transforme &Path en Cow<str>
            // .into_owned() transforme ce cow<str> en String
        ).await;
        assert!(!result); // on attend qu’un dossier normal n’est pas un dépôt Git donc on attend result == false
    }

    #[tokio::test] 
    async fn test_if_git_repository_path_nonexistent(){
        let temp_dir = tempfile::tempdir().unwrap(); // pareil on créer un dossier temporaire par exemple /tmp/.tmpAbC123
        let path_noexistent = temp_dir.path().join("nimportequoi"); // /tmp/.tmpAbC123/nimportequoi le sous dossier n'importe n'est jamais créer

        let result = if_git_repository(
            path_noexistent.to_string_lossy().into_owned()
        ).await;
        assert!(!result)
    }   

    // Scénario : le dossier sélectionner est un dossier git 
    #[tokio::test]
    async fn test_if_git_repository_is_valide(){
        let temp_dir = tempfile::tempdir().unwrap(); // on recréer un dossier temporaire son type est tempfile::TempDir et pas &Path
        git2::Repository::init(temp_dir.path()).unwrap(); // on initie créer le dépot git

        let result = if_git_repository(
            temp_dir.path().to_string_lossy().into_owned() 
            // .path() récupère le chemin du dossier temporaire et renvoie un &Path
            // .to_string_lossy() convertit &Path en Cow<str> 
            // .into_owned() convertit Cow<str>  en string
        ).await;
        assert!(result)
    }
}