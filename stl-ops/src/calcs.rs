use crate::linalg;
use crate::parsing;

pub fn calculate_surface_area(mesh: &parsing::Mesh) -> f32 {
    let triangles = &mesh.triangles;
    let mut area = 0.0;
    
    for triangle in triangles {
        let triangle = *triangle as usize;
        let vertices = mesh.vertices[triangle];
        let tri_area = calculate_triangle_area(vertices);
        area = area + tri_area;
    }
    
    area
}


pub fn calculate_triangle_area(triangle: [[f32; 3]; 3]) -> f32 {

    let a = triangle[0];
    let b = triangle[1];
    let c = triangle[2];

    let ab = [b[0]-a[0], b[1]-a[1], b[2]-a[2]];
    let ac = [c[0]-a[0], c[1]-a[1], c[2]-a[2]];

    let cross = linalg::cross(ab, ac);

    let area = linalg::norm(cross) / 2.0;

    area
}


pub fn calculate_bbox(mesh: parsing::Mesh) -> [f32;3] {
    let vertices = mesh.vertices;

    let mut x_vals = Vec::new();
    let mut y_vals = Vec::new();
    let mut z_vals = Vec::new();

    for triangle in vertices {
        for tri in triangle{
            x_vals.push(tri[0]);
            y_vals.push(tri[1]);
            z_vals.push(tri[2]);
        }
    };

    let (x_min,x_max) = linalg::minmax(x_vals);
    let (y_min,y_max) = linalg::minmax(y_vals);
    let (z_min,z_max) = linalg::minmax(z_vals);

    let x_dim = x_max - x_min;
    let y_dim = y_max - y_min;
    let z_dim = z_max - z_min;

    [x_dim, y_dim, z_dim]

}

pub fn calculate_centroid(mesh: parsing::Mesh) -> Vec<f32> {
    let triangles = mesh.triangles;
    let tri_count : f32 = triangles.len() as f32;
    let mut centroid : Vec<f32> = Vec::new();
    let mut cx : f32 = 0.0;
    let mut cy : f32 = 0.0;
    let mut cz : f32 = 0.0;
    
    for triangle in triangles {
        let triangle = triangle as usize;

        let vertices = mesh.vertices[triangle];
    
        let v1 = vertices[0];
        let v2 = vertices[1];
        let v3 = vertices[2];

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


pub fn calculate_volume(mesh: &parsing::Mesh) -> f32 {
    let triangles = &mesh.triangles;
    let vertices = &mesh.vertices;
    let mut volume = 0.0;
    
    
    for triangle in triangles {
        let triangle = *triangle as usize;
        let tri_verts = vertices[triangle]; 
        let v1 = tri_verts[0];
        let v2 = tri_verts[1];
        let v3 = tri_verts[2];
        let cross = linalg::cross(v1,v2);

        let dot = linalg::dot(cross, v3);

        let v = (1.0 / 6.0) * dot;
        volume = volume + v;
    }
    
    volume
}
