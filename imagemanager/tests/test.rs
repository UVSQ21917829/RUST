
extern crate imagemanager;

#[cfg(test)]
mod tests {  
    #[test]
    fn test_new() {
        let path = "images/info.JPG";
        let meta = imagemanager::image::MetadataImage::new(&path.to_string()).unwrap();
        
    }
    #[test]
    fn test_date() {
        let path = "images/info.JPG";
        let meta = imagemanager::image::MetadataImage::new(&path.to_string()).unwrap();
        assert_eq!(meta.get_image_date().datetime, "2017:05:10 10:30:25");
    }
    #[test]
    fn test_image_type() {
        let _path = "images/info.JPG";
        let meta = imagemanager::image::MetadataImage::new(&_path.to_string()).unwrap();
        assert_eq!(meta.get_image_type(), "image/jpeg");
    }
    
    #[test]
    fn test_gps_longitude() {
        let path = "images/info.JPG";
        let meta = imagemanager::image::MetadataImage::new(&path.to_string()).unwrap();
        assert_eq!(meta.get_image_gps().unwrap().longitude, 2.1487555555555553);
    }
    #[test]
    fn test_gps_latitude() {
        let path = "images/info.JPG";
        let meta = imagemanager::image::MetadataImage::new(&path.to_string()).unwrap();
        assert_eq!(meta.get_image_gps().unwrap().latitude, 48.812886111111105);
    }
    #[test]
    fn test_gps_altitude() {
        let path = "images/info.JPG";
        let meta = imagemanager::image::MetadataImage::new(&path.to_string()).unwrap();
        assert_eq!(meta.get_image_gps().unwrap().altitude, 131.28054298642533);
    }
    
    #[test]
    fn test_iso_speed() {
        let _path = "images/info.JPG";
        let meta = imagemanager::image::MetadataImage::new(&_path.to_string()).unwrap();
        assert_eq!(meta.get_image_iso_speed(), 80);
    }
    #[test]
    fn test_resolution_image() {
        let path = "images/info.JPG";
        let meta = imagemanager::image::MetadataImage::new(&path.to_string()).unwrap();
        assert_eq!(meta.get_image_resolution().x, "72/1");
    }
    
      #[test]
      fn test_supports_exif_fn() {
        let path = "images/info.JPG";
        let meta = imagemanager::image::MetadataImage::new(&path.to_string()).unwrap();
        assert_eq!(meta.supports_exif_fn(),true );
    }
   
    #[test]
    fn test_supports_iptc_fn() {
        let path = "images/info.JPG";
        let meta = imagemanager::image::MetadataImage::new(&path.to_string()).unwrap();
        assert_eq!(meta.supports_iptc_fn(),true );
    }
    #[test]
    fn test_supports_xmp_fn() {
        let path = "images/info.JPG";
        let meta = imagemanager::image::MetadataImage::new(&path.to_string()).unwrap();
        assert_eq!(meta.supports_xmp_fn(),true );
    }
      #[test]
         
      fn test_has_exif_fn() {
        let path = "images/info.JPG";
        let meta = imagemanager::image::MetadataImage::new(&path.to_string()).unwrap();
        assert_eq!(meta.has_exif_fn(),true );
    }
        #[test]
    fn test_has_xmp_fn() {
        let path = "images/info.JPG";
        let meta = imagemanager::image::MetadataImage::new(&path.to_string()).unwrap();
        assert_eq!(meta.has_xmp_fn(),false );
    }
        #[test]
    fn test_has_iptc_fn() {
        let path = "images/info.JPG";
        let meta = imagemanager::image::MetadataImage::new(&path.to_string()).unwrap();
        assert_eq!(meta.has_iptc_fn(),false );
    }
      #[test]
    fn test_get_pixel_width_img() {
        let path = "images/info.JPG";
        let meta = imagemanager::image::MetadataImage::new(&path.to_string()).unwrap();
        assert_eq!(meta.get_pixel_width_img(),3264 );
    }
     #[test]
    fn test_get_pixel_height_img() {
        let path = "images/info.JPG";
        let meta = imagemanager::image::MetadataImage::new(&path.to_string()).unwrap();
        assert_eq!(meta.get_pixel_height_img(),2448 );
    }

      #[test]
    fn test_clear_iptc_fn() {
        let path = "images/img2.jpg";
        let meta = imagemanager::image::MetadataImage::new(&path.to_string()).unwrap();
       if meta.clear_iptc_fn()==true {
            println!("les métadonnées IPTC sont supprimées");
        } else {
            println!("les métadonnées IPTC ne sont pas supprimées");
        }

    }  #[test]
    fn test_clear_exif_fn() {
        let path = "images/img2.jpg";
        let meta = imagemanager::image::MetadataImage::new(&path.to_string()).unwrap();
       if meta.clear_exif_fn()==true {
            println!("les métadonnées Exif sont supprimées");
        } else {
            println!("les métadonnées Exif ne sont pas supprimées");
        }
    }
      #[test]
        fn test_clear_xmp_fn() {
        let path = "images/img2.jpg";
        let meta = imagemanager::image::MetadataImage::new(&path.to_string()).unwrap();
       if meta.clear_xmp_fn()==true {
            println!("les métadonnées XMP sont supprimées");
        } else {
            println!("les métadonnées XMP ne sont pas supprimées");
        }

    }
} 
