fn main() {

    // variables in rust
    let x = 4;
    println!("Hello, x is {}", x);
    {
        let y = 41;
        println!("Hello, y is {}", y);
    }

    let z: u32 = 23423;
    println!("z is {}", z);

    let tup: (i32, bool, char) = (1212, false, 'g');
    println!("tupples, tup is {}", tup.0);
}
