fn main() {
    // Primitive Data Types
    let a: i32 = 5; // signed 32-bit integer
    let b: u32 = 5; // unsigned 32-bit integer
    let c: i64 = 5; // signed 64-bit integer
    let d: u64 = 5; // unsigned 64-bit integer
    let e: f32 = 5.0; // 32-bit floating point
    let f: f64 = 5.0; // 64-bit floating point
    let g: bool = true; // boolean
    let h: char = 'a'; // character

    // Compound Data Types
    let i: &str = "Hello, World!"; // string slice
    let j: String = String::from("Hello, World!"); // string
    let k: [i32; 5] = [1, 2, 3, 4, 5]; // array
    let l: (i32, f64, u8) = (500, 6.4, 1); // tuple
                                           // let m: Fruit = Fruit::new_fruit(); // struct
    let h: Vec<i32> = Vec::new(); // vector
    let n: Option<i32> = Some(5); // option
    let o: Result<i32, i32> = Ok(5); // result
    let p: Box<i32> = Box::new(5); // box
                                   // let q: Rc<i32> = Rc::new(5); // reference counting
}
