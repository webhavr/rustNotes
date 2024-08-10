pub fn example_1() {
    println!("Vaibhav entering example_1");
    let mut x: [i32; 5] = [1, 2, 3, 4, 5];
    x[2] = 4;
    let ref_x: &mut [i32] = &mut x[2..=4];

    
    println!("x : {x:?}");
    println!("x : {:?}", ref_x);
    println!("x : {x:?}");
}