use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    let mut v: Vec<u32> = Vec::new();
    v.push(2);
    v.push(4);
    v.push(5);

    println!("this is a vector {}", v.len());
    let s = do_something_with_vector(&mut v);

    //this is error because v has been moved to the function parameter vector, and then it is dropped
    //println!("this is a vector {}", v.len());

    println!("third element in two ways {}", read_vector(&v))
}

fn do_something_with_vector(vector: &mut Vec<u32>) -> u32 {
    let first_value = vector.pop();
    let fv: u32 = first_value.unwrap_or(2);
    fv
}

fn read_vector(vector: &Vec<u32>) -> u32 {
    //let third: &u32 = &vector[2];
    let something = match vector.get(2) {
        Some(element) => *element,
        None => 2,
    };
    something
}

fn create_map() -> HashMap<String,String> {
    let mut map = HashMap::new();
    map
}
