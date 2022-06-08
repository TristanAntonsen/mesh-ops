mod linalg;
mod calcs;
mod parsing;

fn main() {
    
    let mesh = parsing::import(String::from("../stl/cube_binary.stl"));
    let tricount = mesh.triangles().len();
    let volume = calcs::calculate_volume(&mesh);
    let surface_area = calcs::calculate_surface_area(&mesh);
    let centroid = calcs::calculate_centroid(&mesh);
    
    parsing::create_mesh(mesh);
    println!("Triangle count: {}",tricount);
    println!("Volume: {}",volume);
    println!("Surface Area: {}",surface_area);
    println!("Centroid: {:?}",centroid);


}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
