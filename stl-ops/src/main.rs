mod linalg;
mod calcs;
mod parsing;

fn main() {

    let mesh = parsing::import("../stl/flourite.stl");
    let tricount = mesh.triangles.len();
    let volume = calcs::calculate_volume(&mesh);
    let surface_area = calcs::calculate_surface_area(&mesh);
    let centroid = calcs::calculate_centroid(mesh);
    
    println!("{:?}",tricount);
    println!("Triangle count: {}",tricount);
    println!("Volume: {:.0} mm^3",volume);
    println!("Surface Area: {:.0} mm^2",surface_area);
    println!("Centroid: ({:.3},{:.3},{:.3})",centroid[0],centroid[1],centroid[2]);

}

// fn print_type_of<T>(_: &T) {
//     println!("{}", std::any::type_name::<T>())
// }
