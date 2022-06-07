use std::io::BufReader;
use std::fs::File;
use std::env;

mod linalg;
mod calcs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    // let file = File::open("../stl/flourite.stl").unwrap();
    let file = File::open(filename).unwrap();
    let mut root_vase = BufReader::new(&file);
    let mesh: nom_stl::Mesh = nom_stl::parse_stl(&mut root_vase).unwrap();

    let volume = calcs::calculate_volume(&mesh);
    let centroid = calcs::calculate_centroid(&mesh);
    
    println!("Volume: {}",volume);
    println!("Centroid: {:?}",centroid);
}

