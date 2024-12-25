fn main() {
    // values stored in heap memory
    let s1 = String::from("world");
    let s2 = s1.clone();

    let bg: str = "uuuu";

    // values stored in stack
    let s3 = 3;
    let s4 = s1;
}

// moving ownership to a function

fn main() {
    let vec_1 = vec![1, 2, 3];
    take_ownership(vec_1); // vec_1.clone() to not move
}

fn take_ownership(vec: Vec<i32>) {
    println("{}", vec);
}

// give ownership
