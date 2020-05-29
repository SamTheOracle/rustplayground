fn main() {
    //rust let you decide where to store data, either in the stack or the heap

    /*
    1) Each value in Rust has a variable that’s called its owner.
    2) There can only be one owner at a time.
    3) When the owner goes out of scope, the value will be dropped.
    */

     // s is not valid here, it’s not yet declared
     let s = "hello";   // s is valid from this point forward

     // do stuff with s
    println!("Hello, world!");

    //String is different from &str because it allocates data in the heap
    //Rust takes a different path: the memory is automatically returned once the variable that owns it goes out of scope.
    let string = String::from("hello");

    //a simple copy. 
    let x = 5;
    let y = x;

    //When a variable like String is assigned, there are three parts to consider: pointer, length and capacity.
    //pointer points to an address on the heap, and it is on the stack.
    //when we copy like that it happens that there are two pointers pointing at the same heap
    //this operation is called moving. After that s1 is not valid anymore
    let s1 = String::from("hello");
    let s2 = s1;
    println!(s1);
}//s cannot be user here
