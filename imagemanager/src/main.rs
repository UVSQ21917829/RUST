
use rexiv2::Metadata;

fn main() {
    
let path = "pg.png";
let meta = rexiv2::Metadata::new_from_path(&path).unwrap();
assert_eq!(meta.get_media_type().unwrap(), rexiv2::MediaType::Jpeg);

println!(" time : {:?} !", meta.get_exposure_time());
}
