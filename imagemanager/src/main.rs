use ::std::io;
extern crate imagemanager;
extern crate rexiv2 as rexiv2;

fn main() {
    
    if let Ok(gestionnaire) = imagemanager::image::ImagesToManage::new() {
        loop {
            println!(
                "Veuillez choisir le mode de selection souhaité:
                1: Par nom:
                2: Par date:
                3: Par localisation gps:
                4: Par appariel photos:
                5: Par résolution:
                6: Par tags
                7: quitter: "
            );
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Echec de lire la ligne");

            let input = input
                .trim()
                .parse::<u32>()
                .map_err(|_| format!("{} n'est pas un nombre", input))
                .unwrap();
            let mut images: Vec<&imagemanager::image::MetadataImage> = Vec::new();
            match input {
                1 => {
                    println!("Veuillez entrer un nom valide:");
                    let mut name = String::new();
                    io::stdin()
                        .read_line(&mut name)
                        .expect("Echec de lire la ligne");
                    println!("Le nom est :{}",name);
                    images = gestionnaire.select_by_name(name);
                    imagemanager::image::ImagesToManage::after_select(images);
                }

    
                _ => {
                    println!("entrer un nombre valide " );
                    
                }
            }
        }
    }
}

