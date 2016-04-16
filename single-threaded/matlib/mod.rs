// library of matrix and vector functions
// a 3 component vector
pub struct Vec3{
    pub x: f32,
    pub y: f32,
    pub z: f32
}

// associated functions of Vec3
impl Vec3{
    // used for constructing Vec3 nicely
    pub fn new(x:f32,y:f32,z:f32) -> Vec3 {
        Vec3{x: x, y: y, z: z}
    }
}

// a 3x3 matrix
pub struct Mat3{
    pub values: [[f32;3];3]
}

impl Mat3{
    // used for constructing an empty matrix
    pub fn new_empty() -> Mat3{
        Mat3{values:
            [[0.0,0.0,0.0],
            [0.0,0.0,0.0],
            [0.0,0.0,0.0]]
        }
    }
}

pub fn dot(vec1:&Vec3,vec2:&Vec3) -> f32{
    (vec1.x*vec2.x + vec1.y*vec2.y + vec1.z*vec2.z)
}

// matrix multiplication of a 3x3 matrix with a 3 dimensional vector (1x3 matrix)
// returns the result as a vec3
#[allow(non_snake_case)]
pub fn MatXVec3(mat: &Mat3,vec: &Vec3) -> Vec3{
    let mut result = Vec3::new(0.0,0.0,0.0);

    // construct vectors from the matrix
    let mval = mat.values;
    let mvec1 = Vec3::new(mval[0][0],mval[0][1],mval[0][2]);
    let mvec2 = Vec3::new(mval[1][0],mval[1][1],mval[1][2]);
    let mvec3 = Vec3::new(mval[2][0],mval[2][1],mval[2][2]);

    // matrix multiplication is just a bunch of dot products
    result.x = dot(&mvec1,vec);
    result.y = dot(&mvec2,vec);
    result.z = dot(&mvec3,vec);

    // return resulting vector
    result
}

// Matrix multiplication of a 3x3 matrix with a 3x3 matrix
// returns the result as a Mat3
#[allow(non_snake_case)]
pub fn MatXMat3 (mat1: &Mat3,mat2: &Mat3) -> Mat3{
    let mut result = Mat3::new_empty();

    // go by row
    for i in 0..3{
        // go by column
        for j in 0..3{
            let mut sum :f32 = 0.0;
            // get the result of the row,column pair (i,j)
            for k in 0..3{
                sum = sum + (mat1.values[i][k] * mat2.values[k][j]);
            }
            result.values[j][i] = sum;
        }
    }

    result
}
