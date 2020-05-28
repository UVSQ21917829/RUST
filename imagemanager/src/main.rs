use ::std::io;
extern crate imagemanager;
extern crate rexiv2 as rexiv2;

fn main() {
    let path = "images/info.JPG";
    let meta = imagemanager::image::MetadataImage::new(path.to_string()).unwrap();
    meta.show_xmp_data();
    meta.show_exif_data();
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
                    println!("Le nom est :{}", name);
                    
                    images = gestionnaire.select_by_name(name);
                    imagemanager::image::ImagesToManage::after_select(images);
                }
                2 => {
                    
                        println!("Veuillez entrer une date valide 0000:00:00");
                    let mut date = String::new();
                    io::stdin()
                        .read_line(&mut date)
                        .expect("Echec de lire la ligne");
                    println!("La date est :{}", date);
                    images = gestionnaire.select_by_date(date);
                    imagemanager::image::ImagesToManage::after_select(images);
                }
                3 => {
                    println!("Veuillez longitude");
                    let mut long = String::new();
                    io::stdin()
                        .read_line(&mut long)
                        .expect("Echec de lire la ligne");
                    let long = long
                        .trim()
                        .parse::<f64>()
                        .map_err(|_| format!("{} n'est pas un nombre", long))
                        .unwrap();
                    let mut lat = String::new();
                    io::stdin()
                        .read_line(&mut lat)
                        .expect("Echec de lire la ligne");
                    let lat = lat
                        .trim()
                        .parse::<f64>()
                        .map_err(|_| format!("{} n'est pas un nombre", long))
                        .unwrap();
                    let mut alt = String::new();
                    io::stdin()
                        .read_line(&mut alt)
                        .expect("Echec de lire la ligne");
                    let alt = alt
                        .trim()
                        .parse::<f64>()
                        .map_err(|_| format!("{} n'est pas un nombre", alt))
                        .unwrap();
                    images = gestionnaire.select_by_gps(long, lat, alt);
                    imagemanager::image::ImagesToManage::after_select(images);
                }

                _ => {
                    println!("entrer un nombre valide ");
                }
            }
        }
    }
    
}
