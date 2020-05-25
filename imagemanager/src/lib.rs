#![crate_type = "lib"]

extern crate rexiv2 as rexiv2;

pub use rexiv2::Rexiv2Error;
pub use std::fs;
pub use std::cell::RefCell;
pub mod image {
    #[allow(dead_code)]
    //struct pour les trois dates exif
    pub struct Date {
        pub origin: String,
        pub datetime: String,
        pub digitized: String,
    }
    //struct pour la résolution de l'image
    pub struct Resolution {
        pub x: String,
        pub y: String,
        pub unit: String,
    }
    //struct pour la llocalisation gps de l'image
    pub struct GPS {
        pub longitude: f64,
        pub latitude: f64,
        pub altitude: f64,
    }
    //MetadataImage struct
    #[derive(Debug)]
    pub struct MetadataImage {
        //pub image:  String, //the path of the image
        image: rexiv2::Metadata,
        name: String,
    }
    #[derive(Debug)]
    pub struct ImagesToManage {
        //list of images 
        list: Vec<MetadataImage>,
        
    }

    impl MetadataImage {
        //new method to create a new MetadataImage
        pub fn new(s: String) -> Result<MetadataImage, rexiv2::Rexiv2Error> {
            match rexiv2::Metadata::new_from_path(&s) {
                //let path = Path::new("foo.rs");

                //assert_eq!("foo", path.file_stem().unwrap());
                Ok(metadata) => return Ok(MetadataImage { image: metadata , name:   s}),
                Err(error) => {
                    return Err(error);
                }
            };
        }
        //new method to create a new MetadataImage
        pub fn get_name(&self) -> String {
            
                    return self.name.clone();
              
        }

        //La fonction qui permet de récuperer les dates (et heures)  l'image
        pub fn get_image_date(&self) -> Date {
            let mut date = Date {
                origin: "0".to_string(),
                datetime: "0".to_string(),
                digitized: "0".to_string(),
            };
            if let Ok(time) = self.image.get_tag_string("Exif.Photo.DateTimeOriginal") {
                date.origin = time;
            } else {
                date.origin = "".to_string();
            }
            if let Ok(time) = self.image.get_tag_string("Exif.Image.DateTime") {
                date.datetime = time;
            } else {
                date.datetime = "".to_string();
            }
            if let Ok(time) = self.image.get_tag_string("Exif.Photo.DateTimeDigitized") {
                date.digitized = time;
            } else {
                date.digitized = "".to_string();
            }

            return date;
        }

        //récuperer le type d'image
        pub fn get_image_type(&self) -> String {
            return self.image.get_media_type().unwrap().to_string();
            
        }

        //récuperer la vitesse ISO utilisée par l'appareil photo prenant la photo

        pub fn get_image_iso_speed(&self) -> i32 {
            match self.image.get_iso_speed() {
                Some(speed) => return speed,
                None => return 0,
            };
        }

        //les informations de localosations GPS
        pub fn get_image_gps(&self) -> Option<GPS> {
            match self.image.get_gps_info() {
                Some(loc) => {
                    return Some(GPS {
                        longitude: loc.longitude,
                        latitude: loc.latitude,
                        altitude: loc.altitude,
                    })
                }

                None => None,
            }
        }

        //récuperer les résolution de l'image
        pub fn get_image_resolution(&self) -> Resolution {
            let mut resolution = Resolution {
                x: "".to_string(),
                y: "".to_string(),
                unit: "".to_string(),
            };
            //La résolution X
            if let Ok(resx) = self.image.get_tag_string("Exif.Image.XResolution") {
                resolution.x = resx;
            } else {
                resolution.x = "".to_string();
            }
            // La résolution Y
            if let Ok(resy) = self.image.get_tag_string("Exif.Image.YResolution") {
                resolution.y = resy;
            } else {
                resolution.y = "".to_string();
            }
            // La résolution Unit
            if let Ok(resu) = self.image.get_tag_string("Exif.Image.ResolutionUnit") {
                resolution.unit = resu;
            } else {
                resolution.unit = "".to_string();
            }

            return resolution;
        }

        //Déterminez si le type de fichier chargé prend en charge les métadonnées IPTC.
        pub fn supports_iptc_fn(&self) -> bool {
            if self.image.supports_iptc() == true {
                return true;
            } else {
                return false;
            }
        }

        //Déterminez si le type de fichier chargé prend en charge les métadonnées IPTC.
        pub fn supports_exif_fn(&self) -> bool {
            let mut supports_iptc = true;
            if self.image.supports_iptc() == true {
                return supports_iptc;
            } else {
                supports_iptc = false;
            }
            return supports_iptc;
        }
        //Déterminez si le type de fichier chargé prend en charge les métadonnées XMP.
        pub fn supports_xmp_fn(&self) -> bool {
            let mut supports_xmp = true;
            if self.image.supports_xmp() == true {
                return supports_xmp;
            } else {
                supports_xmp = false;
            }
            return supports_xmp;
        }

        //Indique si le fichier chargé contient des métadonnées Exif.
        pub fn has_exif_fn(&self) -> bool {
            let mut exif = true;

            if self.image.has_exif() == true {
                return exif;
            } else {
                exif = false;
            }
            return exif;
        }
        //Indique si le fichier chargé contient des métadonnées XMP.
        pub fn has_xmp_fn(&self) -> bool {
            let mut xmp = true;
            if self.image.has_xmp() == true {
                return xmp;
            } else {
                xmp = false;
            }
            return xmp;
        }
        //Indique si le fichier chargé contient des métadonnées IPTC.
        pub fn has_iptc_fn(&self) -> bool {
            let mut iptc = true;
            if self.image.has_iptc() == true {
                return iptc;
            } else {
                iptc = false;
            }
            return iptc;
        }
        //Obtenez la largeur réelle de pixels non pivotés / non orientés de l'image chargée.
        pub fn get_pixel_width_img(&self) -> i32 {
            let pix_w = self.image.get_pixel_width();
            return pix_w;
        }

        //Obtenez la hauteur réelle de pixels non pivotés / non orientés de l'image chargée.
        pub fn get_pixel_height_img(&self) -> i32 {
            let pix_h = self.image.get_pixel_height();
            return pix_h;
        }
        //Supprime toutes les métadonnées Exif.
        pub fn clear_exif_fn(&self) -> bool {
            self.image.clear_exif();
            return true;
        }
        //Supprime toutes les métadonnées IPTC.
        pub fn clear_iptc_fn(&self) -> bool {
            self.image.clear_iptc();
            return true;
        }
        //Supprime toutes les métadonnées XMP.
        pub fn clear_xmp_fn(&self) -> bool {
            self.image.clear_xmp();
            return true;
        }
        
        // ************************ afficher EXIF infomatiions for image *********************************
        pub fn show_exif_data(&self) -> () {
            if let Ok(tags) = self.image.get_exif_tags() {
                println!("************* Affichage de EXIF tags *************");
                for i in 0..tags.len() {
                    println!("");
                    if let Ok(xamp_info) = self.image.get_tag_string(&tags[i]) {
                        println!("{:?}   :  {:?}", &tags[i], xamp_info);
                    }
                }
            } else {
                println!("pas de tag exif ");
            }
        }

        // ************************ afficher xmp infomatiions for image *********************************
        pub fn show_xmp_data(&self) -> () {
            if let Ok(tags) = self.image.get_xmp_tags() {
                println!("************* Affichage de XMP tags *************");
                println!("");
                for i in 0..tags.len() {
                    if let Ok(xamp_info) = self.image.get_tag_string(&tags[i]) {
                        println!("{:?}   :  {:?}", &tags[i], xamp_info);
                    }
                }
            } else {
                println!("pas de tag ");
            }
        }

        // ************************ afficher iptc infomatiions for image *********************************
        pub fn show_iptc_data(&self) -> () {
            if let Ok(tags) = self.image.get_iptc_tags() {
                println!("************* Affichage de IPTC tags *************");
                for i in 0..tags.len() {
                    println!("");
                    if let Ok(xamp_info) = self.image.get_tag_string(&tags[i]) {
                        println!("{:?}   :  {:?}", &tags[i], xamp_info);
                    }
                }
            } else {
                println!("pas de tag ");
            }
        }
    }
    impl ImagesToManage {
        //l'initialisation de ImangesManage
        pub fn new()->Result<ImagesToManage,()>{
            let paths = std::fs::read_dir("images").unwrap();
            let mut list :Vec<MetadataImage> = Vec::new();
            for path in paths {
                let pathimg=path.unwrap().path().display().to_string();
                if let  Ok(meta) = MetadataImage::new(pathimg ) {
                    list.push(meta);
                }
            }
            return  Ok(ImagesToManage { list: list  });

        }
        
        pub fn select_by_name(&self,name:String)->Vec<&MetadataImage>{
            let mut images :Vec<& MetadataImage> =  Vec::new();
            for i in 0..self.list.len() {
                if name == self.list[i].get_name(){
                    images.push(&self.list[i]);
                }
            }
            return images;

        }
        pub fn select_by_date(&self,date:String)->Vec<&MetadataImage>{
            let mut images :Vec<&MetadataImage> =  Vec::new();
            for i in 0..self.list.len() {
                let mut s=self.list[i].get_image_date().origin.clone();
                let v: Vec<_> = s.split(' ').collect();
                if date == v[0]{
                    images.push(&self.list[i]);
                }
            }
            return images;
        }
        pub fn select_by_gps(&self,longitude:f64,latitude:f64,altitude:f64)->Vec<&MetadataImage>{
            let mut images :Vec<&MetadataImage> =  Vec::new();
            for i in 0..self.list.len() {
                if let Some(imagemeta)=self.list[i].get_image_gps(){
                if longitude == imagemeta.longitude && latitude == imagemeta.latitude && altitude == imagemeta.altitude{
                    images.push(&self.list[i]);
                }}
            }
            return images;
        }
    }

}
