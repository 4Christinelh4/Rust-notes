fn fn_string(a: String) -> String {
    let res = a.to_ascii_uppercase();
    println!("{res}");
    return res;
}

fn fn_str(a: &str) {
    // wrong - can't mutable borrow
    // a.make_ascii_uppercase();
    let mut mutable_string = a.to_string();
    
    // Step 2: Modify the String
    mutable_string.push_str(": Welcome to Rust!");

    println!("fn_str: {a}")
}

fn fn_mutstr(a: &mut str) {
    a.make_ascii_uppercase();
    println!("fn_mutstr: {a}"); // RUST PROGRAMMING
}  

fn main() {
    let a = "rust programming";

    // shared borrow of a
    let b = String::from(a);

    println!("{a}");
    fn_string(b);

    // b is moved in fn_string, can't print here - owned by fn_string
    // println!("{b}");

    fn_str(a);
    println!("{a}");

    let mut c = "Rust string";
    c = "hey";
    println!("{c}");


    let d: &mut str = &mut String::from(a);
    fn_mutstr(d);
    println!("final a = {a}"); // rust programming
}
