use std::thread;
use std::sync::{Mutex,Arc};
use std::sync::mpsc;

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
#[derive(Copy,Clone)]
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

// compute the dot product of two vectors
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
    let result = Arc::new(Mutex::new(Mat3::new_empty()));
    let val1 = mat1.values; // values in first matrix
    let val2 =  mat2.values; // values in second matrix

    let (tx, rx) = mpsc::channel(); // create a transmitter and receiver pair

    // go by row
    for i in 0..3{
        // go by column
        for j in 0..3{
            // get the result of the row,column pair (i,j)
            // first extract the vectors
            let mvec1 = Vec3::new(val1[i][0],val1[i][1],val1[i][2]); // row from first matrix
            let mvec2 = Vec3::new(val2[0][j],val2[1][j],val2[2][j]); // column from second matrix

            let (result,tx) = (result.clone(),tx.clone());

            // finally, take the dot product of the vectors
            // in their own threads
            thread::spawn(move ||{
                let mut result = result.lock().unwrap();
                result.values[j][i] = dot(&mvec1,&mvec2); // take dot product of row/column pair

                tx.send(()).unwrap(); // transmit something for our receiever to pick up
            });
        }
    }

    // receive the transmissions, signifying those threads have finished
    // when we receive 9 transmissions,
    // all 9 values in the matrix have been computed
    for _ in 0..9{
        rx.recv().unwrap();
    }

    // hack to get the matrix out of the arc+mutex
    // don't try this at home kids!
    let guard = match result.lock(){
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };

    let guard2 = *guard;

    guard2
}
