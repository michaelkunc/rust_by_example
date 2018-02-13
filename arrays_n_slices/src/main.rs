use std::mem

fn analyize_slice(slice: &[i32]){
    println!("The first element of the slices is {}", slice[0]);
    println!("The slice has {} elements", slice.len());
}

fn main() {
    let xs: [i32; 5] = [1,2,3,4,5];
    let ys: [i32; 500] = [0; 500];

}
