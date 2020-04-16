


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
    pub fn get_gps_longitude(&self)-> f64{
        let mut longitude =0.0_f64;
        if let  meta = rexiv2::Metadata::new_from_path(&self.image ).unwrap() {
           if let Some(gps)=meta.get_gps_info(){
           longitude=gps.longitude;
           }
           else{
            longitude=0.0;
           }
        }     
    
        return longitude;
    }
    //recuper gpsinfo: Latitude
    pub fn get_gps_latitude(&self)-> f64{
        let mut latitude =0.0_f64;
        if let  meta = rexiv2::Metadata::new_from_path(&self.image ).unwrap() {
           if let Some(gps)=meta.get_gps_info(){
            latitude=gps.latitude;
           }
           else{
            latitude=0.0;
           }
        }     
        return latitude;
    }
    //recuper gpsinfo: Altitude
    pub fn get_gps_altitude(&self)-> f64{
        let mut altitude =0.0_f64;
        if let  meta = rexiv2::Metadata::new_from_path(&self.image ).unwrap() {
           if let Some(gps)=meta.get_gps_info(){
            altitude=gps.altitude;
           }
           else{
            altitude=0.0;
           }
        }     
        return altitude;
    }
    
}
}
