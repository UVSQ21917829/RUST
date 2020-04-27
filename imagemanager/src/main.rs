

extern crate rexiv2 as rexiv2;

fn main() {
    let _path = "images/info.JPG";
    let meta = imagemanager::image::MetadataImage::new(&_path);
    println!("datetime{:?}", meta.get_image_date().datetime);
    println!("origin{:?}", meta.get_image_date().origin);
    println!("digitized{:?}", meta.get_image_date().digitized);
    println!("x{:?}", meta.get_image_resolution().x);
    println!("y{:?}", meta.get_image_resolution().y);
    println!("unit{:?}", meta.get_image_resolution().unit);
    //println!("flash{:?}", meta.get_image_flash());
    //println!("modele{:?}", meta.get_image_model());
    //println!("software{:?}", meta.get_image_software());
    //println!("orientation{:?}", meta.get_image_orientation());
    //println!("orientation{:?}", meta.get_image_orientation_n());
    //println!("orientation{:?}", meta.get_image_orientation_n());
    let  metae = rexiv2::Metadata::new_from_path(&_path ).unwrap();
    println!("xmp tags{:?}", metae.get_xmp_tags());
    println!("iptc{:?}", metae.get_iptc_tags());
    println!("has iptc{:?}", metae.has_iptc());
   // meta.show_xmp_data();
    //meta.show_iptc_data();
    
}

