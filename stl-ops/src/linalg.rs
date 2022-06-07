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
