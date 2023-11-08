fn main() {
    // Write a program to find the sum of natural numbers below a given number N, where N is provided by the user. 
    // The sum should only include numbers that are multiples of either 3 or 5.
    // For example, if the user enters N = 20, the multiples of 3 are 3, 6, 9, 12, 15, 18, and the multiples of 5 are 5, 10, and 15. 
    // Please note that the value of 15 will be considered only once since it is a multiple of both 3 and 5.
    // The sum will be calculated as follows: 3 + 5 + 6 + 9 + 10 + 12 + 15 + 18.
    // Write a program that takes the user input N, performs the necessary calculations, and outputs the sum.

    let mut number = String::new();
    std::io::stdin()
        .read_line(&mut number)
        .expect("Failed to read input.");

    let number: u128 = number.trim().parse().expect("Invalid input");

    println!("The final calculate the difference is {}", sum(number));
}

fn sum(mut num: u128) -> u128 {
    let mut number:u128 = 0;

    while num > 0 {
        num = num - 1;
        if num % 3 == 0 || num % 5 == 0{
           if num % 3 == 0 && num % 5 == 0 {
            number += num;
            continue;
           }
           number += num;
        }
    }

    return number;
}
