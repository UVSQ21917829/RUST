

#![crate_type = "lib"]

extern crate rexiv2 as rexiv2;

pub mod image{
#[allow(dead_code)]
//struct pour les trois dates exif
pub struct Date {
    pub origin: String,
    pub datetime: String,
    pub digitized: String
}
//struct pour la résolution de l'image
pub struct Resolution {
    pub x: String,
    pub y: String,
    pub unit: String
}
//struct pour la llocalisation gps de l'image
pub struct GPS {
    pub longitude: f64,
    pub latitude: f64,
    pub altitude: f64
}
//MetadataImage struct 
pub struct MetadataImage {
    pub image:  String, //the path of the image 

}

impl MetadataImage{

    //new method to create a new MetadataImage 
    pub fn new(s: &str)->MetadataImage{
 
        return MetadataImage { image: s.to_string() }
    }
    
    //La fonction qui permet de récuperer les dates (et heures)  l'image
    pub fn get_image_date(&self)-> Date{
        let mut date = Date{origin:"0".to_string(),
         datetime: "0".to_string(),
         digitized:"0".to_string(),};
        if let  Ok(meta) = rexiv2::Metadata::new_from_path(&self.image ) {
            if let Ok(time) = meta.get_tag_string("Exif.Photo.DateTimeOriginal") {
                date.origin=time;
            }else{
                date.origin="".to_string();
            }
            if let Ok(time) = meta.get_tag_string("Exif.Image.DateTime") {
                date.datetime=time;
            }else{
                date.datetime="".to_string();
            }
            if let Ok(time) = meta.get_tag_string("Exif.Photo.DateTimeDigitized") {
                date.digitized=time;
            }else{
                date.digitized="".to_string();
            }
        }
        return date;
    }
    //récuperer le type d'image
    pub fn get_image_type(&self)-> String{
        let mut typee = String::new();
        if let  Ok(meta) = rexiv2::Metadata::new_from_path(&self.image ) {
            typee= meta.get_media_type().unwrap().to_string();

        }
        return typee;
    }
    //récuperer la vitesse ISO utilisée par l'appareil photo prenant la photo

    pub fn get_image_iso_speed(&self)-> i32{
        let mut speed = 0_i32;
        if let  Ok(meta) = rexiv2::Metadata::new_from_path(&self.image ) {
            if let  Some(v) = meta.get_iso_speed() {
                speed=v;
            }else{
                speed=0;
            }
        }
        return speed;
    }
    
    //les informations de localosations GPS
    pub fn get_image_gps(&self)-> GPS{
        let mut gps =GPS{
            longitude:0.0_f64,
            latitude:0.0_f64,
            altitude:0.0_f64,

        };
        if let  Ok(meta) = rexiv2::Metadata::new_from_path(&self.image ) {
           if let Some(loc)=meta.get_gps_info(){
           gps.longitude=loc.longitude;
           gps.latitude=loc.latitude;
           gps.altitude=loc.altitude;
           }
           
        }     
    
        return gps;
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
  //Supprime toutes les métadonnées IPTC.
 pub fn clear_iptc_fn(&self)-> bool{
  let  meta = rexiv2::Metadata::new_from_path(&self.image ).unwrap() ;
  meta.clear_iptc();
  return true;
 }
  //Supprime toutes les métadonnées XMP.
 pub fn clear_xmp_fn(&self)-> bool{
  let  meta = rexiv2::Metadata::new_from_path(&self.image ).unwrap() ;
  meta.clear_xmp();
  return true;
 }

}
}
