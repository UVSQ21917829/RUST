use std::io;
extern crate imagemanager;
extern crate rexiv2 as rexiv2;

fn main() {
    if let Ok(gestionnaire) = imagemanager::image::ImagesToManage::new() {
        loop {
            println!(
                "Veuillez choisir le mode de selection souhaité:
                1: Par nom
                2: Par date
                3: Par localisation gps
                4: Par appareil photos
                5: Par résolution
                6: Par tags
                7: quitter "
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
                    let name = name.trim();
                    //println!("name:{:?}", name);
                    images = gestionnaire.select_by_name(name.to_string());
                    imagemanager::image::ImagesToManage::print_all(&images);
                    imagemanager::image::ImagesToManage::after_select(&images);
                }
                2 => {
                    println!("Veuillez entrer une date valide 0000:00:00");
                    let mut date = String::new();
                    io::stdin()
                        .read_line(&mut date)
                        .expect("Echec de lire la ligne");
                    let date = date.trim();
                    images = gestionnaire.select_by_date(date.to_string());
                    imagemanager::image::ImagesToManage::print_all(&images);
                    imagemanager::image::ImagesToManage::after_select(&images);
                }
                3 => {
                    println!("Veuillez entrer la valeur longitude");
                    let mut long = String::new();
                    io::stdin()
                        .read_line(&mut long)
                        .expect("Echec de lire la ligne");
                    let mut lon = 0f64;
                       
                        match long.trim().parse::<f64>() {
                          Ok(n) => {
                            lon=n;
                          }
                          Err(e) => println!("La longitude doit être un nombre")
                        }
                    println!("Veuillez entrer la valeur latitude");
                    let mut lati = String::new();
                    io::stdin()
                        .read_line(&mut lati)
                        .expect("Echec de lire la ligne");
                    let mut lat = 0f64;
                       
                        match lati.trim().parse::<f64>() {
                          Ok(n) => {
                            lat=n;

                          }
                          Err(e) => println!("La longitude doit être un nombre")
                        }
                    println!("Veuillez entrer la valeur altitude");
                    let mut alti = String::new();
                    io::stdin()
                        .read_line(&mut alti)
                        .expect("Echec de lire la ligne");
                    let mut alt = 0f64;
                        match alti.trim().parse::<f64>() {
                          Ok(n) => {
                            alt=n;
                          }
                          Err(e) => println!("La longitude doit être un nombre")
                        }
                    images = gestionnaire.select_by_gps(lon, lat, alt);
                    imagemanager::image::ImagesToManage::print_all(&images);
                    imagemanager::image::ImagesToManage::after_select(&images);
                }
                4 => {
                    println!("Veuillez entrer le nom d appareil");
                    let mut app = String::new();
                    io::stdin()
                        .read_line(&mut app)
                        .expect("Echec de lire la ligne");
                    let app = app.trim();
                    images = gestionnaire.select_by_camera(app.to_string());
                    imagemanager::image::ImagesToManage::print_all(&images);
                    imagemanager::image::ImagesToManage::after_select(&images);
                }
                5 => {
                    println!("Veuillez entrer la resolotion x");
                    let mut x = String::new();
                    io::stdin()
                        .read_line(&mut x)
                        .expect("Echec de lire la ligne");
                    let x = x.trim();
                    println!("Veuillez entrer la resolotion y");
                    let mut y = String::new();
                    io::stdin()
                        .read_line(&mut y)
                        .expect("Echec de lire la ligne");
                    let y = y.trim();
                    images = gestionnaire.select_by_resolutio(x.to_string(), y.to_string());
                    imagemanager::image::ImagesToManage::print_all(&images);
                    imagemanager::image::ImagesToManage::after_select(&images);
                }
                6 => {
                    println!("Veuillez entrer une expression");
                    let mut tag = String::new();

                    io::stdin()
                        .read_line(&mut tag)
                        .expect("Echec de lire la ligne");
                    let tag = tag.trim();
                    images = gestionnaire.select_by_tag(tag.to_string());
                    imagemanager::image::ImagesToManage::print_all(&images);
                    imagemanager::image::ImagesToManage::after_select(&images);
                }
                7 => {
                    println!("Vous avez quité l'application ");
                    break;
                }
                _ => {
                    println!("Entrer un nombre valide 1,2,3,3,4,5,6 ou 7 ");
                }
            }
        }
    }
}
