use std::fmt;

#[derive(Debug)]
struct MinMax(i64, i64);

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "({}, {} )", self.0, self.1)
    }
}

#[derive(Debug)]
struct Point2d{
    x: f64,
    y: f64,
}

impl fmt::Display for Point2d {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

#[derive(Debug)]
struct Complex{
    real: f64,
    imag: i32,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f,"real: {}, imag: {}", self.real, self.imag)
    }
}
fn main(){
    let minmax = MinMax(0, 14);

    println!("Compare Structures:");
    println!("Display {}", minmax);
    println!("Debug {:?}", minmax);

    let point = Point2d{x: 3.3, y: 7.2};
    println!("Compare points");
    println!("Display {}", point);
    println!("Debug {:?}", point);

    let complex = Complex{real: 3.3, imag: 15};
    println!("Compare points");
    println!("Display {}", complex);
    println!("Debug {:?}", complex);
}