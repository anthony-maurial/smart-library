use crate::livre::{ Livre};

pub struct Bibliotheque {
    nom_librairie: String,
    nombre_livre: u32,
    adresse: String,
    livre_list: Vec<Livre>,
}

impl Bibliotheque {
    pub fn new(un_nom: String, une_adresse:String) -> Bibliotheque {
        Bibliotheque {
            nom_librairie: un_nom,
            adresse: une_adresse,
            nombre_livre: 0,
            livre_list: Vec::new(),
        }
    }

    pub fn regarder_livre(&self) {
        println!("les livres présents dans la bibliotheque sont : ");
        for l in &self.livre_list {
            println!("nom: {}, auteur: {}, genre: {}, nombre de page: {}", l.nom.trim(), l.auteur.trim(), l.genre.types_str(), l.nb_page);
        }
    }

    pub fn ajouter_livre(&mut self, livre: Livre){
        self.nombre_livre += 1;
        self.livre_list.push(livre);
    }

    pub fn retirer_livre(&mut self){
        if self.nombre_livre != 0 {
            self.nombre_livre -= 1;
            self.livre_list.pop();
        }
    }

    pub fn to_stringg(&self) -> String{
        let res = format!("nom: {}, nombre de livre: {}, adresse: {}",self.nom_librairie.to_string(), self.nombre_livre.to_string(), self.adresse.to_string());
        res
    }
}