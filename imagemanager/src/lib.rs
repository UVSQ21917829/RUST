
/*#[derive(Debug, PartialEq)]
pub struct Metadata {
    raw: *mut rexiv2::Metadata,
}
impl Metadata{

    pub fn get_imagedate_time(&self)-> Result<String>{
        if let Ok(time) = meta.get_tag_string("Exif.Image.DateTime") {
            ///println!("Time: {:?}", time);
            Ok(time);
        }
    }
}
*/
use std::path::Path;
use std::ffi::OsStr;

fn get_extension_from_filename(filename: &str) -> Option<&str> {
    Path::new(filename)
        .extension()
        .and_then(OsStr::to_str)
}