fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x
    let z = &y; // z is an immutable reference to y (which is mutable)

    *y += 1; // Modify x through y
    println!("x = {}", x); // Output: x = 6

    // This will not compile because you are trying to change x 
    // through a reference (z) that is immutable
    //*z += 1;  
}