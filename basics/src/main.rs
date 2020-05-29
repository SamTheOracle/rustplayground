

fn main() {
    //variables are by default immutable
    let mut x = 12;
    //
    x = 6;
    //consta are same as let but need to specify the type. Can be declared globally 
    const constant:u32 =12;


    //this constructs allows to use same variable name.
    let spaces = "   ";
    let spaces = spaces.len();
    

    //this is compile error, because the same variable type is changed
    // let mut spaces = "";
    // spaces = 12;



}

