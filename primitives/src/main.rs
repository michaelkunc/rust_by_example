fn main() {
    let logical: bool = true;
    let a_float: f64 = 1.0;
    let an_integer = 5i32;
    let default_float = 5.0;
    let default_int = 7;
    let mut inferred_type = 12;
    println!("{}", inferred_type);
    inferred_type = 55;

    let mut mutable = 2;
    mutable = 10;
    let mutable = true;

    println!("{}", logical);
    println!("{}", a_float);
    println!("{}", an_integer);
    println!("{}", default_float);
    println!("{}", default_int);
    println!("{}", inferred_type);
}
