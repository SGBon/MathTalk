// insert contents of matlib/mod.rs into a module "matlib"
mod matlib;

fn main(){
    // create our test matrix
    let mat_test1 = matlib::Mat3{
        values: [
        [1.4,2.4,3.1],
        [1.2,4.9,3.0],
        [0.1,0.4,3.7]]
    };

    // create a matrix to do Matrix by Matrix product
    let mat_test2 = matlib::Mat3{
        values: [
        [2.3,1.0,4.2],
        [1.7,4.6,0.4],
        [6.2,0.5,1.0]]
    };

    // create some test vectors
    let vect1 = matlib::Vec3::new(2.0, 1.3,3.2);
    let vect2 = matlib::Vec3::new(1.4,1.2,4.3);
    let vect3 = matlib::Vec3::new(0.5,0.3,2.7);

    // multiply the matrix with the vector
    let vec_product = matlib::MatXVec3(&mat_test1,&vect1);
    let mat_product = matlib::MatXMat3(&mat_test1,&mat_test2);

    println!("the dot product is {}",matlib::dot(&vect2,&vect3));
    println!("the matrix-vector product is {} {} {}",vec_product.x,vec_product.y,vec_product.z);

    // print out the matrix-matrix product
    println!("the matrix-matrix product is:");
    for i in 0..3{
        println!("{} {} {}",mat_product.values[0][i],mat_product.values[1][i],mat_product.values[2][i])
    }

    // scale and rotate a 3d point
    let point = matlib::Vec3::new(1.4,3.7,0.9);
    let model = matlib::MatXMat3(&matlib::Mat3::new_scale(2.0,1.0,3.7),&matlib::Mat3::new_rotate_y(32.0));

    let srpoint = matlib::MatXVec3(&model,&point);
    println!("Our point in space {} {} {}",point.x,point.y,point.z);
    println!("Is now {} {} {}",srpoint.x,srpoint.y,srpoint.z);
}
