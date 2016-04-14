// a 3 component vector
#[allow(dead_code)]
struct Vec3{
    x: f32,
    y: f32,
    z: f32
}

// a 3x3 matrix
#[allow(dead_code)]
struct Mat3{
    values: [f32;9]
}

fn dot(vec1:&Vec3,vec2:&Vec3) -> f32{
    (vec1.x*vec2.x + vec1.y*vec2.y + vec1.z*vec2.z)
}

#[allow(non_snake_case)]
fn MatXVec3(mat: &Mat3,vec: &Vec3) -> Vec3{
    let mut result = Vec3{x:0.0,y:0.0,z:0.0};

    // construct vectors from the matrix
    let mval = mat.values;
    let mvec1 = Vec3{x:mval[0],y:mval[1],z:mval[2]};
    let mvec2 = Vec3{x:mval[3],y:mval[4],z:mval[5]};
    let mvec3 = Vec3{x:mval[6],y:mval[7],z:mval[8]};

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

    let vect = Vec3{x:2.0, y:1.3,z:3.2};
    let vect2 = Vec3{x:1.4,y:1.2,z:4.3};
    let vect3 = Vec3{x:0.5,y:0.3,z:2.7};

    let product = MatXVec3(&mat_test,&vect);
    println!("the result is {} {} {}",product.x,product.y,product.z);
    println!("the dot product is {}",dot(&vect2,&vect3));
}
