
extern crate imagemanager;

#[cfg(test)]
mod tests {  
    #[test]
    fn test_new() {
        let path = "info.JPG";
        let meta = imagemanager::image::MetadataImage::new(&path);
        assert_eq!(meta.image, "info.JPG");
    }
    #[test]
    fn test_date() {
        let path = "images/info.JPG";
        let meta = imagemanager::image::MetadataImage::new(&path);
        assert_eq!(meta.get_image_date_time(), "2017:05:10 10:30:25");
    }
    #[test]
    fn test_gps_longitude() {
        let path = "images/info.JPG";
        let meta = imagemanager::image::MetadataImage::new(&path);
        assert_eq!(meta.get_gps_longitude(), 2.1487555555555553);
    }
    #[test]
    fn test_gps_latitude() {
        let path = "images/info.JPG";
        let meta = imagemanager::image::MetadataImage::new(&path);
        assert_eq!(meta.get_gps_latitude(), 48.812886111111105);
    }
} 
