fn main() {
    let mut x = 5;
    {
        let y = &mut x; // y is a mutable reference to x
        *y = 10;
        println!("x = {}", x); // Output: x = 10
    }

    let z = &x; // This is now valid because the mutable reference has gone out of scope.
    println!("x = {}", *z); // Output: x = 10
} 