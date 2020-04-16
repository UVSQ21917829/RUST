
extern crate imagemanager;

#[cfg(test)]
mod tests {  
    #[test]
    fn test_new() {
        let path = "info.JPG";
        let meta = imagemanager::image::MetadataImage::new(&path);
        assert_eq!(meta.image, "info.JPG");
    }
      
} 
