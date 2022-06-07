use std::io::BufReader;
use std::fs::File;
use std::env;

mod linalg;
mod calcs;

fn main() {
    
    let mesh = import(String::from("../stl/suzanne.stl"));
    let volume = calcs::calculate_volume(&mesh);
    let centroid = calcs::calculate_centroid(&mesh);
    
    println!("Volume: {}",volume);
    println!("Centroid: {:?}",centroid);
}

fn import(path: String) -> nom_stl::Mesh {
    let args: Vec<String> = env::args().collect();
    // let filename = &args[1];
    // let file = File::open(filename).unwrap();
    let file = File::open(path).unwrap();
    let mut root_vase = BufReader::new(&file);
    let mesh: nom_stl::Mesh = nom_stl::parse_stl(&mut root_vase).unwrap();
    mesh
}
