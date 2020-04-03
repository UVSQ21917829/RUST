extern crate rexiv2;
#[test]
fn test_type() {
    let path = "pg.png";
    let meta = rexiv2::Metadata::new_from_path(&path).unwrap();
    assert_eq!(meta.get_media_type().unwrap(), rexiv2::MediaType::Png);
}
#[test]
fn test_time_exposure() {
let meta = rexiv2::Metadata::new_from_path(&path).unwrap();
let path = "pg.png";
println!(" time : {:?} !", meta.get_exposure_time());

}
#[test]
fn test_gps() {
   
let meta = rexiv2::Metadata::new_from_path(&path).unwrap();
let path = "pg.png";
println!(" pgs : {:?} !", meta.get_gps_info());
}
