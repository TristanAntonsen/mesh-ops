use std::io::BufReader;
use std::fs::File;
use std::env;

fn main() {

    // let args: Vec<String> = env::args().collect();
    // let filename = &args[1];


    let file = File::open("../stl/cube_binary.stl").unwrap();
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
    let tri_count : i32 = triangles.len() as i32;    
    let volume = 0.0;
    
    for triangle in triangles {
        let tri_verts = triangle.vertices(); 
        let v1 = tri_verts[0];
        let v2 = tri_verts[1];
        let v3 = tri_verts[2];
        let cross = cross(v1,v3);

        // let dot = dot(&cross, v3);


        println!("Vector 1:      {:?}",tri_verts[0]);
        println!("Vector 2:      {:?}",tri_verts[1]);
        println!("Cross product: {:?}",cross);
    }


    volume
}


// fn calc_tet_volume(triangle: Vec<f32>) -> f32 {
//     let v = (1 / 6) * 
// }

fn cross(A : [f32; 3], B : [f32; 3]) -> [f32; 3] {


    let i = A[1] * B[2] - A[2] * B[1];

    let j = A[2] * B[0] - A[0] * B[2];

    let k = A[0] * B[1] - A[1] * B[0];

    let cross = [i, j, k];

    cross

}

fn dot(A : [f32; 3], B : [f32; 3]) -> [f32; 3] {
    let i = A[0] * B[0];
    let j = A[1] * B[1];
    let k = A[2] * B[2];

    let dot = [i, j, k];

    dot
}


fn calculate_centroid(mesh: nom_stl::Mesh) -> Vec<f32> {
    let triangles = mesh.triangles();
    let tri_count : f32 = triangles.len() as f32;
    let mut centroid : Vec<f32> = Vec::new();
    let mut cx : f32 = 0.0;
    let mut cy : f32 = 0.0;
    let mut cz : f32 = 0.0;
    
    for triangle in triangles {
        

        let v1 = triangle.vertices()[0];
        let v2 = triangle.vertices()[1];
        let v3 = triangle.vertices()[2];

        let x_avg = (v1[0] + v2[0] + v3[0]) / 3.0;
        let y_avg = (v1[1] + v2[1] + v3[1]) / 3.0;
        let z_avg = (v1[2] + v2[2] + v3[2]) / 3.0;

        cx = cx + x_avg as f32;
        cy = cy + y_avg as f32;
        cz = cz + z_avg as f32;
    }

    cx = cx / tri_count;
    cy = cy / tri_count;
    cz = cz / tri_count;

    centroid.push(cx);
    centroid.push(cy);
    centroid.push(cz);

    centroid
}