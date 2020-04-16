

#![crate_type = "lib"]
use std::path::Path;

use std::fs;
use std::ptr;
extern crate rexiv2 as rexiv2;

pub mod image{
#[allow(dead_code)]
//MetadataImage struct 
pub struct MetadataImage {
    pub image:  String, //the path of the image 

}

impl MetadataImage{

    //new method to create a new MetadataImage 
    pub fn new(s: &str)->MetadataImage{
 
        return MetadataImage { image: s.to_string() }
    }
    //La fonction qui permet de récuperer la date et le temps de créer l'image
    pub fn get_image_date_time(&self)-> String{
        let mut date = String::new();
        if let  Ok(meta) = rexiv2::Metadata::new_from_path(&self.image ) {
            if let Ok(time) = meta.get_tag_string("Exif.Image.DateTime") {
                date=time;
            }else{
                date="".to_string();
            }
        }
        return date;
    }
    //recuper gpsinfo: longitude
    pub fn get_gps_longitude(&self)-> String{
        let mut longitude = String::new();
        if let  Ok(meta) = rexiv2::Metadata::new_from_path(&self.image ) {
            if let Ok(longps) = meta.get_tag_string("Exif.GPSInfo.GPSLongitude") {
            longitude=longps;
            }else{
                longitude="".to_string();
            }
        }
        return longitude;
    }
    
}
}
