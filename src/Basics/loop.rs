fn main() {
    'outer: loop {
        println!("Entered the outer loop");
        break 'outer;
    }

    let vec: Vec<i32> = vec![3, 4, 5, 4, 2];

    for i in vec {
        println!("The value is: {}", i);
    }

    let mut num: i32 = 0;

    while num < 10 {
        num = num + 1;
    }
    println!("The value is: {}", num);
}
