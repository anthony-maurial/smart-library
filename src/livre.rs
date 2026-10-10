use crate::types::Types;

pub struct Livre {
    pub nom: String,
    pub genre: Types,
    pub auteur: String,
    pub nb_page: u32,
}

impl Livre {
    pub fn new(un_nom: String, un_auteur: String, un_genre: Types, un_nb_page: u32) -> Livre{
        Livre{
            nom: un_nom,
            auteur: un_auteur,
            genre: un_genre,
            nb_page: un_nb_page,
        }
    }
    pub fn new_livre_int(un_nom: String, un_auteur: String, un_genre: i8, un_nb_page: u32) -> Livre{
        match un_genre {
            1 => Livre::new(un_nom, un_auteur, Types::Roman, un_nb_page),
            2 => Livre::new(un_nom, un_auteur, Types::Romance, un_nb_page),
            3 => Livre::new(un_nom, un_auteur, Types::Manga, un_nb_page),
            4 => Livre::new(un_nom, un_auteur, Types::BandeDessine, un_nb_page),
            5 => Livre::new(un_nom, un_auteur, Types::ScienceFiction, un_nb_page),
            6 => Livre::new(un_nom, un_auteur, Types::Horreur, un_nb_page),
            7 => Livre::new(un_nom, un_auteur, Types::Fantasy, un_nb_page),
            8 => Livre::new(un_nom, un_auteur, Types::Science, un_nb_page),
            9 => Livre::new(un_nom, un_auteur, Types::Histoire, un_nb_page),
            10 => Livre::new(un_nom, un_auteur, Types::Scolaire, un_nb_page),
            _ => {
                println!("Livre inconnu, on met un Roman par défaut");
                Livre::new(un_nom, un_auteur, Types::Roman, un_nb_page)
            },
        }
    }
    
}