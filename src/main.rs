unsafe extern "C" {
    fn say_hello();
    fn add(a: i32, b: i32) -> i32;
}

fn main() {
    unsafe {
        say_hello();
        let sum = add(3, 5);
        println!("Sum from C: {}", sum);
    }
}
