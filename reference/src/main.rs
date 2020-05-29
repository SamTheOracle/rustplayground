/*

The Rules of References

1) At any given time, you can have either one mutable reference or any number of immutable references.
2) References must always be valid.

*/

fn main() {
    let s1 = String::from("hello");

    // we pass the reference of the value s1. allows to refer some values without having ownership.bool
    //the value it points to will not be dropped after scope ends
    //reference is function parameters are known as borrowings.

    //we are not allowed to mutate references' values
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);


    //we can use "mut" to mutate the value the reference points to
    let mut s = String::from("ciao");
    let test = change(&mut s);
    //restriction: only a mutable reference to the same data for each scope
    //let r1 = &mut s
    //let r2 = &mut s
    //(does not compile)

    //restriction: cannot have one immutable reference and a mutable one at the same time, unless one is used before the other

    //this piece of code compiles because despite previous restriction, immutable references are already used
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // r1 and r2 are no longer used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);

    // let reference_to_nothing = dangle();
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
fn change (s:&mut String){
    s.push_str("2");
}

//this does not compile because it returns a reference to nothing. s goes out of scope and it is dropped, so &s points to nothing (dangling reference)
// fn dangle() -> &String {
//     let s = String::from("hello");

//     &s
// }

//this function works because ownership is moved to the variable that holds no_dangle
fn no_dangle() -> String {
    let s = String::from("hello");

    s
}