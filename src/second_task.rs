fn main() {
    // Write a program to find the difference between the square of the sum and the sum of the squares of the first N numbers. 
    // N will be a user-defined input that your program will take.
    // For example, if the user enters the number 5, you should compute the square of the sum as (1 + 2 + 3 + 4 + 5)^2 = 225.
    // Next, compute the sum of the squares as (1^2 + 2^2 + 3^2 + 4^2 + 5^2) = (1 + 4 + 9 + 16 + 25) = 55.
    // Finally, calculate the difference as 225 - 55 = 170.

    let mut number = String::new();
    std::io::stdin()
        .read_line(&mut number)
        .expect("Failed to read input.");

    let number: u128 = number.trim().parse().expect("Invalid input");

    println!("The final calculated difference is {}", get_diff(number));
}

fn get_diff(mut num: u128) -> u128 {
    let mut number:u128 = 0;
    let mut number0:u128 = 0;

    while num > 0 {
        number += num;
        number0 += num.pow(2);
        num = num - 1;
    }

    if number.pow(2) > number0{
        return number.pow(2) - number0
    }else{
        return number0 - number.pow(2)
    }
}