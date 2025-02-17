fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x
    let z = &x;   // z is an immutable reference to x

    // This is valid because x is modified through y which is mutable.
    *y = 10;
    println!("x = {}", x); // Output: x = 10

    // This is invalid.  The compiler will not allow us to create z after y (mutable reference) is defined
    // because it could potentially invalidate z. 
    // let z = &x; // This line will cause a compile time error
}