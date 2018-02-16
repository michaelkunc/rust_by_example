#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

struct Nil;

struct Pair(i32, f32);

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug)]
struct Rectangle{
    p1: Point,
    p2: Point,
}

// TODO: implement rect_area
fn rect_area(rect: &Rectangle)-> f32{
    let Point {x: x1, y: y1} = rect.p1;
    let Point {x: x2, y: y2} = rect.p2;
    (x1 - x2) * (y1-y2)
}

// TODO: implement square
fn main() {
    let name = "Peter";
    let age = 40;
    let peter = Person {name, age};
    println!("{:?}", peter);

    let point = Point {x: 0.3, y: 0.4};
    println!("point coordinates are {} {}", point.x, point.y);

    let new_points = Point {x : 0.1, ..point};
    println!("new point coordinates are {} {}", new_points.x, new_points.y);
    
    let Point {x: my_x, y: my_y} = point;
    println!("I don't know if this will work {} {}", my_x, my_y);

    let _rectangle = Rectangle {
        p1: Point {x: my_x, y: my_y},
        p2: point,
    };

    let _nil = Nil;

    let pair = Pair(1, 0.1);
    println!("pair contains {:?} {:?}", pair.0, pair.1);

    let Pair(integer, decimal) = pair;
    println!("deconstructedp pair contains {} {}", integer, decimal);

    println!("the rectangle points are {:?} {:?}", _rectangle.p1, _rectangle.p2);

    println!("the area of the rectange is {:?}", rect_area(&_rectangle));
    
}
