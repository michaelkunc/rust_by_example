use std::mem;

fn analyize_slice(slice: &[i32]){
    println!("The first element of the slices is {}", slice[0]);
    println!("The slice has {} elements", slice.len());
}

fn main() {
    let xs: [i32; 5] = [1,2,3,4,5];
    let ys: [i32; 500] = [0; 500];

    println!("This first element is {}", xs[0]);
    println!("The second element is {}", xs[1]);

    println!("array size {}", xs.len());

    println!("Array occupies {} bytes", mem::size_of_val(&xs));

    println!("Borrow the whole array as a slice \n");
    analyize_slice(&xs);

    println!("Borrow a section as a slice \n");
    analyize_slice(&ys[1..4]);

    println!("{}", xs[5]);
}
