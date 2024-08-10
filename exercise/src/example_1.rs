struct Rectangle {
    length: u16,
    width: u16,
}

impl Rectangle {

    fn area(&self) -> u16 {
        self.length * self.width
    }

}

pub fn example_1() {
    println!("Vaibhav entering example_1");

    let rect = Rectangle {length: 3, width: 4};
    println!("Area = {}", rect.area());

}