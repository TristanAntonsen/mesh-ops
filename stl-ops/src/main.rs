mod linalg;
mod calcs;
mod parsing;

fn main() {
    
    let mesh = parsing::import(String::from("../stl/suzanne.stl"));
    let volume = calcs::calculate_volume(&mesh);
    let centroid = calcs::calculate_centroid(&mesh);
    
    println!("Volume: {}",volume);
    println!("Centroid: {:?}",centroid);
}

