


use rexiv2::Metadata;
use std::path::Path;
use std::fs;
fn main() {

    
let path = "images";
if Path::new(&path).exists() {

    
    let paths = fs::read_dir("images").unwrap();

    for path in paths {
        let pathimg=path.unwrap().path().display().to_string();
        
        if let  Ok(meta) = Metadata::new_from_path(&pathimg ) {
            println!("___________________________ Image : _______________________________" );
            if let Ok(typee) = meta.get_media_type() {
                println!("Type d'image: {}", typee);
            }
            if let  Ok(time) = meta.get_tag_string("Exif.Image.DateTime") {
                println!("Time: {:?}", time);
            }
            if let Some(location) = meta.get_gps_info() {
                println!("Localisation: {:?}", location);
            }
            
            if let  pix_h = meta.get_pixel_height() {
                println!("Pixel height: {}", pix_h);
            }
            if let  pix_w = meta.get_pixel_width() {
                println!("Pixel Width: {}", pix_w);
            }
            if let  Some(speed) = meta.get_iso_speed() {
                println!("iso speed: {}", speed);
            }
          
            
        }else
        {
            println!("votre fichier n'est une image ");
        }

    }  

    
}else {
    println!("Erreur Vérifier votre path!!");
}

}
