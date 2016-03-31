// structs can be defined implicity
struct Pair(i32,i32);

fn main(){

    let pair = Pair(20,30);
    // destructure the struct
    let Pair(first,second) = pair;
    println!("{:?} and {:?}",first,second);

}
