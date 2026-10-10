use std::{io::stdin, u8};

use crate::{bibliotheque::Bibliotheque, livre::Livre};

mod types;
mod livre;
mod bibliotheque;

fn menu_type(){
    println!("1. Roman");
    println!("2. Romance");
    println!("3. Manga");
    println!("4. Bande déssiné");
    println!("5. Science fiction");
    println!("6. Horreur");
    println!("7. Fantaisie");
    println!("8. Science");
    println!("9. Histoire");
    println!("10. Scolaire");
}

fn ajouter_livre(lib : &mut Bibliotheque){
    let mut nom: String = String::new();
    let mut auteur: String = String::new();
    let mut entree_temp: String = String::new();
    let mut nb_page_temp: String = String::new();
    println!("Quel est le nom du livre que vous voulez créé ?");
    stdin().read_line(&mut nom).unwrap();
    println!("Quel est le nom de l'auteur du livre que vous voulez créé");
    stdin().read_line(&mut auteur).unwrap();
    println!("Quel est le nombre de page du livre que vous souhaitez crée");
    stdin().read_line(&mut nb_page_temp).unwrap();
    println!("De quel type est votre oeuvre ?");
    menu_type();
    stdin().read_line(&mut entree_temp).unwrap();
    let id : i8 = entree_temp.trim().parse().unwrap();
    let nb_page : u32 = nb_page_temp.trim().parse().unwrap();
    let l : Livre = livre::Livre::new_livre_int(nom, auteur, id, nb_page);
    lib.ajouter_livre(l);
}

fn retirer_livre(lib : &mut Bibliotheque) {
    lib.retirer_livre();
}

fn menu_librairie(lib_list : &[Bibliotheque]) -> i32{
    let mut cpt : i32 = 1;
    for i in lib_list {
        println!("{}. {}", cpt, i.to_stringg());
        cpt +=1;
    }
    cpt
}

fn change_librairie(lib_list : &[Bibliotheque]) -> usize{
    let mut librairie_nom : String = String::new();
    menu_librairie(lib_list);
    stdin().read_line(&mut librairie_nom).unwrap();
    librairie_nom.trim().parse().unwrap()
}

fn cree_librairie(lib : &mut Vec<Bibliotheque>) -> usize{
    let mut nom_librairie : String = String::new();
    let mut adresse : String = String::new();
    println!("Veuillez saisir le nom de la bibliotheque");
    stdin().read_line(&mut nom_librairie).unwrap();
    println!("Veuillez saisir l'adresse de la bibliotheque");
    stdin().read_line(&mut adresse).unwrap();
    lib.push(Bibliotheque::new(nom_librairie, adresse));
    let cpt : i32 = menu_librairie(lib);
    cpt as usize
}

fn regarder_librairie(lib : &Bibliotheque){
    lib.regarder_livre();
}

fn menu() {
    println!("Quel est action voulais-vous faire");
    println!("1. Ajouter un livre");
    println!("2. Retirer un livre");
    println!("3. Changer de bibliotheque");
    println!("4. Crée une bibliotheque");
    println!("5. description de la bibliotheque courante");
    println!("6. Quitter");
}

fn main() {
    let mut fini:bool = false;
    let mut lib_list: Vec<bibliotheque::Bibliotheque> = Vec::new();
    let lib1: Bibliotheque = bibliotheque::Bibliotheque::new("mollat".to_string(), "Bordeaux".to_string());
    lib_list.push(lib1);
    let mut lib_courante: usize = 0;
    while !fini {
        menu();
        let mut input = String::new();
        stdin().read_line(&mut input).expect("il y a une erreur");
        let choix :u8 = input.trim().parse::<u8>().expect("l'entrée est invalide");
        match choix {
            1=> ajouter_livre(&mut lib_list[lib_courante]),
            2=> retirer_livre(&mut lib_list[lib_courante]),
            3=> lib_courante = change_librairie(&lib_list) - 1,
            4=> lib_courante = cree_librairie(&mut  lib_list),
            5=> regarder_librairie(&lib_list[lib_courante]),
            _=> fini = true,
        };
    }
}
