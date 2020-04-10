
use rexiv2::Metadata;
use std::path::Path;
fn main() {
    
let path = "info.JPG";

if Path::new(&path).exists() {


    if let Ok(meta) = Metadata::new_from_path(&path ) {
        if let Ok(time) = meta.get_tag_string("Exif.Image.DateTime") {
            println!("Time: {:?}", time);
        }
        
    }
    
let meta = rexiv2::Metadata::new_from_path(&path).unwrap();
println!("Le type de fichier est : {}",meta.get_media_type().unwrap());
println!(" time : {:?} !", meta.get_exposure_time());
println!(" pgs : {:?} !", meta.get_gps_info());
println!("pxels {}",meta.get_pixel_height());
println!("{:?}",meta.get_exif_tags());
    
}else {
    println!("votre fichier n'existe pas");
}

}
