mod linalg;
mod calcs;
mod parsing;
use std::collections::HashMap;

fn main() {
    
    let mesh = parsing::import(String::from("../stl/sphere.stl"));
    let tricount = mesh.triangles().len();
    let volume = calcs::calculate_volume(&mesh);
    let surface_area = calcs::calculate_surface_area(&mesh);
    let centroid = calcs::calculate_centroid(&mesh);
    
    
    println!("Triangle count: {}",tricount);
    println!("Volume: {}",volume);
    println!("Surface Area: {}",surface_area);
    println!("Centroid: {:?}",centroid);


}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
