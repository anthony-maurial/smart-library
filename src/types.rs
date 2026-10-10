pub enum Types {
    Roman,
    Romance,
    Manga,
    BandeDessine,
    ScienceFiction,
    Horreur,
    Fantasy,
    Science,
    Histoire,
    Scolaire,
}

impl Types {
    pub fn types_str(&self) -> String{
        match self {
            Types::Roman => "Roman".to_string(),
            Types::Romance => "Romance".to_string(),
            Types::Manga => "Manga".to_string(),
            Types::BandeDessine => "Bande déssiné".to_string(),
            Types::ScienceFiction => "Science fiction".to_string(),
            Types::Horreur => "Horreur".to_string(),
            Types::Fantasy => "Fantasy".to_string(),
            Types::Science => "Science".to_string(),
            Types::Histoire => "Histoire".to_string(),
            Types::Scolaire => "Scolaire".to_string(),
        }
    }


//     pub fn nom_type(num: i8) -> String{
//         let mut res : String = String::new();
//         match num {
//             1 => res = String::from("Roman"),
//             2 => res = String::from("Romance"),
//             3 => res = String::from("Manga"),
//             4 => res = String::from("Bande déssiné"),
//             5 => res = String::from("Science fiction"),
//             6 => res = String::from("Horreur"),
//             7 => res = String::from("fantasie"),
//             8 => res = String::from("Science"),
//             9 => res = String::from("Histoire"),
//             10 => res = String::from("scolaire"),
//             _ => res = String::from("Inconnu"),
//         }
//         return res;
//     }
}