// a 3 component vector
struct Vec3{
    x: f32,
    y: f32,
    z: f32
}

// associated functions of Vec3
impl Vec3{
    // used for constructing Vec3 nicely
    fn new(x:f32,y:f32,z:f32) -> Vec3 {
        Vec3{x: x, y: y, z: z}
    }
}

// a 3x3 matrix
struct Mat3{
    values: [f32;9]
}

fn dot(vec1:&Vec3,vec2:&Vec3) -> f32{
    (vec1.x*vec2.x + vec1.y*vec2.y + vec1.z*vec2.z)
}

// matrix multiplication of a 3x3 matrix with a 3 dimensional vector (1x3 matrix)
// returns the result as a vec3
#[allow(non_snake_case)]
fn MatXVec3(mat: &Mat3,vec: &Vec3) -> Vec3{
    let mut result = Vec3::new(0.0,0.0,0.0);

    // construct vectors from the matrix
    let mval = mat.values;
    let mvec1 = Vec3::new(mval[0],mval[1],mval[2]);
    let mvec2 = Vec3::new(mval[3],mval[4],mval[5]);
    let mvec3 = Vec3::new(mval[6],mval[7],mval[8]);

    // matrix multiplication is just a bunch of dot products
    result.x = dot(&mvec1,vec);
    result.y = dot(&mvec2,vec);
    result.y = dot(&mvec3,vec);

    result
}

fn main(){
    let mat_test = Mat3{
        values: [
        1.0,0.0,0.0,
        0.0,1.0,0.0,
        0.0,0.0,1.0]
    };

    // create some test vectors
    let vect1 = Vec3::new(2.0, 1.3,3.2);
    let vect2 = Vec3::new(1.4,1.2,4.3);
    let vect3 = Vec3::new(0.5,0.3,2.7);

    // multiply the matrix with the vector
    let product = MatXVec3(&mat_test,&vect1);

    println!("the dot product is {}",dot(&vect2,&vect3));
    println!("the matrix-vector product is is {} {} {}",product.x,product.y,product.z);
}
