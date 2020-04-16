
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
        let path = "info.JPG";
        let meta = imagemanager::image::MetadataImage::new(&path);
        assert_eq!(meta.get_image_date_time(), "2017:05:10 10:30:25");
    }
} 
