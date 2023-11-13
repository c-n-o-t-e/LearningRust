use std::env;

fn main() {
    // Write a function that implements the logic, 'You can see the movie if you are 17 or older, or if you are 13 or older and have a parent's permission.'

    let args: Vec<String> = env::args().collect();

    let num: i32 = args[1].parse().unwrap();
    let flag: bool = args[2].parse().unwrap();

    println!("Can see a movie {}", can_see_movie(num, flag));
}

fn can_see_movie(age: i32, permission: bool) -> bool {
    (age >= 17) || (age >= 13 && permission)
}