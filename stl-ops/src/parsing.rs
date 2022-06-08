use std::io::BufReader;
use std::fs::File;
use std::collections::HashMap;
// use std::env;

pub fn import(path: String) -> nom_stl::Mesh {
    // let args: Vec<String> = env::args().collect();
    // let filename = &args[1];
    // let file = File::open(filename).unwrap();
    let file = File::open(path).unwrap();
    let mut root_vase = BufReader::new(&file);
    let mesh: nom_stl::Mesh = nom_stl::parse_stl(&mut root_vase).unwrap();
    mesh
}

pub fn create_mesh(nom_mesh: nom_stl::Mesh) -> HashMap<i32, [[f32; 3]; 3]> {

    let nom_tris = nom_mesh.triangles();
    let mut tris = HashMap::new();
    let mut i = 0;

    for triangle in nom_tris {
        let vertices = triangle.vertices();
        tris.insert(i, vertices);
        i = i + 1;
    }


    // println!("{:?}",vertex_ids.get(&1))
    // for (key, value) in &tris {
    //     println!("{}: {:?}",key, value);
    // }

    tris    

}

// Structure for mesh:
// vertex id hash map: {id : [x, y, z]}
// edge hash map: {id: [vertex id1, vertex id2]}