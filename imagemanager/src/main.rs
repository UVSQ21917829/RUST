
use rexiv2::Metadata;
use std::path::Path;
fn main() {


    
let path = "exemple.jpeg";

if Path::new(&path).exists() {
    
let meta = rexiv2::Metadata::new_from_path(&path).unwrap();
println!("Le type de fichier est : {}",meta.get_media_type().unwrap());
println!(" time : {:?} !", meta.get_exposure_time());
println!(" pgs : {:?} !", meta.get_gps_info());
    
}else {
    println!("votre fichier n'existe pas");
}

}
