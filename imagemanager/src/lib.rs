

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
    
}
}
