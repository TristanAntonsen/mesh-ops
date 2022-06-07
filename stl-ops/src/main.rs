use std::io::BufReader;
use std::fs::File;
// use std::env;
extern crate stl_ops;

fn main() {
    // let args: Vec<String> = env::args().collect();
    // let filename = &args[1];


    let file = File::open("../stl/sphere.stl").unwrap();
    // let file = File::open(filename).unwrap();
    let mut root_vase = BufReader::new(&file);
    let mesh: nom_stl::Mesh = nom_stl::parse_stl(&mut root_vase).unwrap();

    // println!("Calculating centroid. . .");

    // let centroid = calculate_centroid(mesh);

    // println!("Centroid: {:?}", centroid)

    calculate_volume(mesh);
}

fn calculate_volume(mesh: nom_stl::Mesh) -> f32 {
    let triangles = mesh.triangles();
    let mut volume = 0.0;
    
    for triangle in triangles {
        let tri_verts = triangle.vertices(); 
        let v1 = tri_verts[0];
        let v2 = tri_verts[1];
        let v3 = tri_verts[2];
        let cross = stl_ops::linalg::cross(v1,v2);

        let dot = stl_ops::linalg::dot(cross, v3);

        let v = (1.0 / 6.0) * dot;
        volume = volume + v;
    }
    
    println!("Volume: {}", volume);

    volume
}


