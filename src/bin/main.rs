use serde::{Serialize,Deserialize};
use std::fs::File;
use std::io::{self, Read, Seek, Write};

#[derive(Serialize, Deserialize, Debug)]
struct Eleve {
    nom: String,
    prenom: String,
    age: u32,
    classe: String,
}


struct Fichier {
    fichier : File,
}

fn main() -> io::Result<()> {

    

    let mut mon_fichier = Fichier::ouverture_fichier("iut.json")?;
    

    // Affichage d'un menu général

    afficher_menu();

    let stdin = io::stdin();                                //récupération et lecture des lignes entrées
    for line in stdin.lines() {
        match line {
            Ok(l) => {
                let words_entry: Vec<_> = l.split_whitespace().collect();
                
                if words_entry[0].is_empty() {                      
                    println!("C'est vide !");
                    println!("Utilisez une commande du menu");
                }
                else {
                    match words_entry[0].to_lowercase().as_str() {
                        "afficher" => {                                           // Affichage des données
                            mon_fichier.afficher_eleve();
                        }
                        "ajout" => {                                              // Ajout d'un élèves dans le fichier
                            mon_fichier.ajout_eleve(words_entry);
                        }
                        "supp" => {                                               // Suppression d'un élève du fichier
                            //mon_fichier.suppression_eleve();
                        }
                        _ => {
                            println!("Veuillez choisir une option valide.");
                        }
                    }
                }
                
            }
            Err(err) => {
            }
        }
    }

    Ok(())
}




// Fonction pour ouvrir le fichier 

impl Fichier {


    // Fonction qui ouvr ele fichier .json
    fn ouverture_fichier(nom: &str) -> io::Result<Self> {
        let mut fichier = File::open(nom)?;
        Ok(Fichier { fichier })
    }



    // Fonction pour ajouter un élève

    fn ajout_eleve(&mut self, words_entry: Vec<&str>) -> io::Result<()> {

        // Vérifier qu'on a bien 5 mots


        // On récuère les valeurs entrées
        print!("coucou 1");
        let nom = words_entry[1];
        let prenom = words_entry[2];
        let age: u32 = match words_entry[3].parse() {
            Ok(age) => age,
            Err(_) => {
                println!("L'âge doit être un nombre entier.");
                return Ok(());
            }
        };
        let classe = words_entry[4];
        print!("coucou 2");
        

        self.fichier.seek(std::io::SeekFrom::Start(0))?;
        let mut content_of_json = String::new();
        self.fichier.read_to_string(&mut content_of_json).expect("Impossible de lire le fichier");
        print!("coucou 3");

        // Suite : 


        // LE GPT 
        /* 
        let mut data: serde_json::Value = match serde_json::from_str(&content_of_json) {
            Ok(data) => data,
            Err(err) => {
                eprintln!("Erreur lors de la conversion JSON : {}", err);
                return Ok(());
            }
        };
        print!("coucou 3");
        let etudiants = data
            .as_object_mut()
            .and_then(|obj| obj.get_mut("etudiants"))
            .and_then(|etudiants| etudiants.as_array_mut());
        print!("coucou 3");
        if let Some(etudiants) = etudiants {
            let nouvel_eleve = serde_json::json!({
                "nom": nom,
                "prenom": prenom,
                "age": age,
                "classe": classe,
            });
    
            etudiants.push(nouvel_eleve);
        } else {
            eprintln!("La clé 'etudiants' est manquante ou invalide dans le fichier JSON.");
            return Ok(());
        }
        print!("coucou 3");
        let new_content = serde_json::to_string_pretty(&data)?;
        print!("coucou 3");
        self.fichier.seek(std::io::SeekFrom::Start(0))?;  // Remettre la tête de lecture au début
        self.fichier.write_all(new_content.as_bytes())?;
    
        Ok(())
        */







        
        // MON CODE 
        /*
        let mut liste_eleves: Vec<Eleve> = serde_json::from_str(&content_of_json)?;
        print!("coucou 4");
        let nouvel_eleve = Eleve {nom: nom.to_string(), prenom: prenom.to_string(), age, classe: classe.to_string()};
        print!("coucou 5");
        liste_eleves.push(nouvel_eleve);
        print!("coucou 6");


        let new_content = serde_json::to_string(&liste_eleves)?;

        self.fichier.seek(std::io::SeekFrom::Start(0))?;  // Remettre la tête de lecture au début
        self.fichier.write_all(new_content.as_bytes())?;
        
        Ok(())
        */
    }




    // Supprimer un élève

    fn suppression_eleve() {

            // A VENIR
    }




    // Consulter le contenu

    fn afficher_eleve(&mut self) -> io::Result<()> {

        self.fichier.seek(std::io::SeekFrom::Start(0))?;  // Remettre la tête de lecture au début
        let mut content_of_json = String::new();
        self.fichier.read_to_string(&mut content_of_json)?;

        // Désérialiser le contenu du fichier JSON
        let data: serde_json::Value = match serde_json::from_str(&content_of_json) {
            Ok(data) => data,
            Err(err) => {
                eprintln!("Erreur lors de la conversion JSON : {}", err);
                return Ok(());
            }
        };

        // Vérifier si la clé "etudiants" existe dans l'objet JSON
        if let Some(etudiants) = data.get("etudiants") {
            // Désérialiser la séquence d'étudiants
            let liste_eleves: Vec<Eleve> = match serde_json::from_value(etudiants.clone()) {
                Ok(eleves) => eleves,
                Err(err) => {
                    eprintln!("Erreur lors de la conversion JSON des étudiants : {}", err);
                    return Ok(());
                }
            };

            // Afficher les détails des étudiants
            for eleve in liste_eleves.iter() {
                println!("Nom : {}", eleve.nom);
                println!("Prénom : {}", eleve.prenom);
                println!("Âge : {}", eleve.age);
                println!("Classe : {}\n", eleve.classe);
            }
        } else {
            eprintln!("La clé 'etudiants' est manquante dans le fichier JSON.");
        }

        Ok(())
    }
}

fn afficher_menu() {
    println!("Bienvenue sur notre application ! \n");
    println!("Que souhaitez-vous faire ? ");
    println!("1 - Afficher les élèves : afficher");
    println!("2 - Ajouter un élève : ajout Nom Prenom Age Classe");
    println!("3 - Supprimer un élève  : supp Nom Prenom");
}

fn lire_contenu_fichier(file: &mut File) -> io::Result<String> {
    let mut content_of_json = String::new();
    file.seek(std::io::SeekFrom::Start(0))?;
    file.read_to_string(&mut content_of_json)?;
    Ok(content_of_json)
}