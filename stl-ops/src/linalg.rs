//Cross product of two 3D vectors
pub fn cross(a : [f32; 3], b : [f32; 3]) -> [f32; 3] {
    
    let i = a[1] * b[2] - a[2] * b[1];
    
    let j = a[0] * b[2] - a[2] * b[0];
    
    let k = a[0] * b[1] - a[1] * b[0];
    
    let cross = [i, -j, k];
    
    cross
    
}

//Dot product of two 3D vectors
pub fn dot(a : [f32; 3], b : [f32; 3]) -> f32 {
    let i = a[0] * b[0];

    let j = a[1] * b[1];

    let k = a[2] * b[2];
    
    let dot = i + j + k;
    
    dot
}

//Magnitude of vector

pub fn norm(v: [f32;3]) -> f32 {

    let a = v[0] as f32;
    let b = v[1] as f32;
    let c = v[2] as f32;

    let a = f32::powi(a,2);
    let b = f32::powi(b,2);
    let c = f32::powi(c,2);

    let sum = a + b + c;

    let norm = sum.sqrt();

    norm

}