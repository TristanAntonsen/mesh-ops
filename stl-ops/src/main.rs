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
    // println!("{:?}",mesh.triangles());
    // let mut mesh_map = HashMap::new();

    let mut i = 0;
    while i < tricount {
        let triangle = mesh.triangles()[i];
        let normal = triangle.normal();
        let vertices = triangle.vertices();
        i = i + 1;
    }
    
    
    // print_type_of(&mesh_map);
    println!("Triangle count: {}",tricount);
    println!("Volume: {}",volume);
    println!("Surface Area: {}",surface_area);
    println!("Centroid: {:?}",centroid);


}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
