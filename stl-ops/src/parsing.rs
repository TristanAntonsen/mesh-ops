use std::io::BufReader;
use std::fs::File;
// use std::env;

pub fn import(path: &str) -> Mesh {
    let path = String::from(path);
    // let args: Vec<String> = env::args().collect();
    // let filename = &args[1];
    // let file = File::open(filename).unwrap();
    let file = File::open(path).unwrap();
    let mut root_vase = BufReader::new(&file);
    let nom_mesh: nom_stl::Mesh = nom_stl::parse_stl(&mut root_vase).unwrap();
    let mesh = create_mesh(nom_mesh);

    mesh
}

pub fn create_mesh(nom_mesh: nom_stl::Mesh) -> Mesh {

    let nom_tris = nom_mesh.triangles();
    let mut i = 0;
    let mut id_vector: Vec<i32> = Vec::new();
    let mut vertex_vector: Vec<[[f32; 3]; 3]> = Vec::new();

    for triangle in nom_tris {
        let vertices = triangle.vertices();
        id_vector.push(i);
        vertex_vector.push(vertices);
        i = i + 1;
        // println!("{}{:?}",i,vertices)
    }

    let new_mesh = Mesh {
        triangles: id_vector,
        vertices: vertex_vector
    };

    new_mesh    

}

pub struct Mesh {
    pub triangles: Vec<i32>,
    pub vertices: Vec<[[f32; 3]; 3]>,
}

// Structure for mesh:
// vertex id hash map: {id : [x, y, z]}
// edge hash map: {id: [vertex id1, vertex id2]}

// pub fn export_ascii_stl(mesh: &Mesh) {
//     let mut content = String::from("");
//     let vertices = &mesh.vertices;
//     content = content + "solid test_stl";
//     for triangle in vertices {
//         let v1 = triangle[0];
//         let v2 = triangle[1];
//         let v3 = triangle[2];
        
//         content = content + "facet normal 0 0 0\n";
//         content = content + "    outer loop\n";
//         content = content + format!("        vertex {} {} {}\n",v1[0],v1[1],v1[2]); 
//         // content = content + format!("        vertex {} {} {}\n",v2[0],v2[1],v2[2]); 
//         // content = content + format!("        vertex {} {} {}\n",v3[0],v3[1],v3[2]); 
//         content = content + "    end loop\n";
//         content = content + "endfacet\n";

//     }


//     std::fs::write("test.txt", content).expect("could not write file");
// }
