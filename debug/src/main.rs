#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

fn main(){
    let name = "Sonny";
    let age = 20;
    let sonny = Person{name, age};

    println!("{:#?}", sonny )

}