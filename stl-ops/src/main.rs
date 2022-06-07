use std::io::BufReader;
use std::fs::File;

mod linalg;
mod calcs;

fn main() {

    let file = File::open("../stl/sphere.stl").unwrap();
    let mut root_vase = BufReader::new(&file);
    let mesh: nom_stl::Mesh = nom_stl::parse_stl(&mut root_vase).unwrap();

    let volume = calcs::calculate_volume(&mesh);
    let centroid = calcs::calculate_centroid(&mesh);
    
    println!("Volume: {}",volume);
    println!("Centroid: {:?}",centroid);
}

