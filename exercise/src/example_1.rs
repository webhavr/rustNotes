pub fn example_1() {
    println!("Vaibhav entering example_1");
    let mut x: i32 = 6;
    print!("{x}", x);
    while x != 1 {
        if x % 2 == 0 {
            x = x/2;
        }
        else {
            x = 3 * x +1;
        }
        print!(" -> {}", x)
    }
    println!();
}