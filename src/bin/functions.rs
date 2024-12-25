fn main() {
    my_fn("Hello, World!");

    let anwser: i32 = single_return(5, 5);
    let (a, b) = multiple_return(5, 5);

    println!("{}", anwser);
    print!("{}, {}", a, b);

    let full_name: String = {
        let first_name: &str = "John";
        let last_name: &str = "Doe";
        format!("{} {}", first_name, last_name)
    }; //code blocks

    print!("{}", full_name);
}

fn my_fn(s: &str) -> () {
    println!("{}", s);
}

fn single_return(a: i32, b: i32) -> i32 {
    a * b
}

fn multiple_return(a: i32, b: i32) -> (i32, i32) {
    (a, b)
}
