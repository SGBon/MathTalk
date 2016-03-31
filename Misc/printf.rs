// c-style formatted printing

fn main(){
    println!("{} days", 31);

    // arguments in formatted print can be reffered to by position.
    // very handy
    println!("{0}, this is {1}. {1}, this is {0}","Alice","Bob");

    // arguments can be named
    println!("{msg}, {name}",msg="Welcome to rust",name="new rust programmer");

    // radix
    println!("{} decimal, {:b} binary, {:x} hex",1000,1000,1000);

}
