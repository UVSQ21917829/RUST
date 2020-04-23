


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
    
 //Déterminez si le type de fichier chargé prend en charge les métadonnées IPTC.
 pub fn supports_iptc_fn(&self) -> bool{
  let mut supports_iptc=true;
  if let  meta = rexiv2::Metadata::new_from_path(&self.image ).unwrap() {
   if meta.supports_iptc()==true {
           return supports_iptc;
       }else{
            supports_iptc=false;
           }
}  return supports_iptc;
 }

 //Déterminez si le type de fichier chargé prend en charge les métadonnées IPTC.
 pub fn supports_exif_fn(&self) -> bool{
  let mut supports_iptc=true;
  if let  meta = rexiv2::Metadata::new_from_path(&self.image ).unwrap() {
   if meta.supports_iptc()==true {
           return supports_iptc;
       }else{
            supports_iptc=false;
           }
}  return supports_iptc;
 }
 //Déterminez si le type de fichier chargé prend en charge les métadonnées XMP.
 pub fn supports_xmp_fn(&self) -> bool{
let mut supports_xmp=true;
  if let  meta = rexiv2::Metadata::new_from_path(&self.image ).unwrap() {
   if meta.supports_xmp()==true {
           return supports_xmp;
       }else{
            supports_xmp=false;
           }
}  return supports_xmp;
 }
  //Indique si le fichier chargé contient des métadonnées Exif.
 pub fn has_exif_fn(&self) -> bool{
let mut exif=true;
  if let  meta = rexiv2::Metadata::new_from_path(&self.image ).unwrap() {
   if meta.has_exif()==true {
           return exif;
       }else{
            exif=false;
           }
}  return exif;
 }
 //Indique si le fichier chargé contient des métadonnées XMP.
 pub fn has_xmp_fn(&self) -> bool{
let mut xmp=true;
  if let  meta = rexiv2::Metadata::new_from_path(&self.image ).unwrap() {
   if meta.has_xmp()==true {
           return xmp;
       }else{
            xmp=false;
           }
}  return xmp;
 }
 //Indique si le fichier chargé contient des métadonnées IPTC.
 pub fn has_iptc_fn(&self) -> bool{
let mut iptc=true;
  if let  meta = rexiv2::Metadata::new_from_path(&self.image ).unwrap() {
   if meta.has_iptc()==true {
           return iptc;
       }else{
            iptc=false;
           }
}  return iptc;
 }
//Obtenez la largeur réelle de pixels non pivotés / non orientés de l'image chargée.
 pub fn get_pixel_width_img(&self) -> i32{
          
         let  meta = rexiv2::Metadata::new_from_path(&self.image ).unwrap() ;
            let  pix_w  = meta.get_pixel_width() ;
            return pix_w;
        }
}
 //Obtenez la hauteur réelle de pixels non pivotés / non orientés de l'image chargée.
 pub fn get_pixel_height_img(&self) -> i32{
         let  meta = rexiv2::Metadata::new_from_path(&self.image ).unwrap() ;
            let  pix_h  = meta.get_pixel_height() ;
           return pix_h;
            
 }
  //Supprime toutes les métadonnées Exif.
 pub fn clear_exif_fn(&self)-> bool{
  let  meta = rexiv2::Metadata::new_from_path(&self.image ).unwrap() ;
  meta.clear_exif();
  return true;
 }
}
