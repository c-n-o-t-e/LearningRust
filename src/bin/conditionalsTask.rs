// Problem 1:

/*
Write a program to find the difference between the square of the sum and the sum of the squares of the first N numbers.
N will be a user-defined input that your program will take.

For example, if the user enters the number 5, you should compute the square of the sum as (1 + 2 + 3 + 4 + 5)^2 = 225.
Next, compute the sum of the squares as (1^2 + 2^2 + 3^2 + 4^2 + 5^2) = (1 + 4 + 9 + 16 + 25) = 55.
Finally, calculate the difference as 225 - 55 = 170.
*/

fn main() {
    let mut n = String::new();
    std::io::stdin()
        .read_line(&mut n)
        .expect("failed to read input.");
    let n: i32 = n.trim().parse().expect("invalid input");

    let mut square_of_sum = 0;
    let mut sum_of_squares = 0;

    // Solution

    for i in 1..=n {
        square_of_sum = square_of_sum + i;
        sum_of_squares = sum_of_squares + i.pow(2);
    }

    let difference = square_of_sum.pow(2) - sum_of_squares;
    println!(
        "The difference of the square_of_sum and sum of Squares for N = {} is {}",
        n, difference
    );
}

// Problem 2:

/*
Write a program to find the sum of natural numbers below a given number N, where N is provided by the user.
The sum should only include numbers that are multiples of either 3 or 5.
For example, if the user enters N = 20, the multiples of 3 are 3, 6, 9, 12, 15, 18, and the multiples of 5 are 5, 10, and 15.

Please note that the value of 15 will be considered only once since it is a multiple of both 3 and 5.
The sum will be calculated as follows: 3 + 5 + 6 + 9 + 10 + 12 + 15 + 18.

Write a program that takes the user input N, performs the necessary calculations, and outputs the sum.
*/

// fn main() {
//     let mut n = String::new();
//     std::io::stdin()
//         .read_line(&mut n)
//         .expect("failed to read input.");
//     let n: i32 = n.trim().parse().expect("invalid input");

//     /* Add your code below this line */
//     let mut sum: i32 = 0;

//     for i in 1..n {
//         if i % 3 == 0 || i % 5 == 0 {
//             sum = sum + i;
//         }
//     }

//     println!("\n\n The sum of the multiples are = {sum}");
// }

// // Problem 3:

// /*
// This question involves writing code to analyze the production of an assembly line in a car factory.
// The assembly line has different speeds, ranging from 0 (off) to 10 (maximum).
// At the lowest speed of 1, the assembly line produces a total of 221 cars per hour.
// The production rate increases linearly with the speed,
// meaning that a speed of 4 produces 4 * 221 = 884 cars per hour.

// However, higher speeds increase the likelihood of producing faulty cars that need to be discarded.
// The success rate depends on the speed, as shown in the table below:
// · Speeds 1 to 4: 100% success rate.
// · Speeds 5 to 8: 90% success rate.
// · Speeds 9 and 10: 77% success rate.

// You need to write two functions:
// 1. The first function, total_production(), calculates the total number of cars successfully produced without faults within a specified time given in hours. The function takes the number of hours and speed as input and returns the number of cars successfully produced.
// 2. The second function, cars_produced_per_minute(), calculates the number of cars successfully produced per minute. The function takes the number of hours and speed as input and returns the number of cars produced per minute.
// Write the code for both functions based on the provided specifications.
// */
// fn main() {
//     println!("{}", total_production(6, 9) as i32); // to round the values we use i32. just ignore for mow
//     println!("{}", cars_produced_per_minutes(6, 9) as i32); // to round the values we use i32. just ignore for mow
// }

// fn total_production(hours: u8, speed: u8) -> f32 {
//     /* Your code below this line*/
//     221.0 * (speed as f32) * (hours as f32) * get_success_rate(speed)
// }

// fn cars_produced_per_minutes(hours: u8, speed: u8) -> f32 {
//     /* Your code below this line*/
//     (221.0 / 60.0) * (speed as f32) * get_success_rate(speed)
// }

// fn get_success_rate(speed: u8) -> f32 {
//     match speed {
//         9..=10 => 0.77,
//         5..=8 => 0.90,
//         1..=4 => 1.0,
//         _ => 0.0,
//     };
// }

// // Problem 4:

// /*
// A palindrome is a word, verse, or sentence that reads the same backward or forward,
// such as 'Able was I ere I saw Elba,' or a number like 1881.

// Write a function named is_palindrome() that checks whether a given string is a palindrome or not.
// The function should take a string as input and return a boolean value indicating whether the string is a palindrome or not.
// */
// fn main() {
//     let input = String::from("1881");
//     println!(
//         "It is {:?} that the given string is palindrome",
//         palindrome(input)
//     );
// }

// fn palindrome(input: String) -> bool {
//     /* Your Code here */
//     let reversed: String = input.chars().rev().collect();
//     input == reversed
// }

// // Problem 5:

// /*
// A Pythagorean triple consists of three positive integers a, b, and c, satisfying the condition a^2 + b^2 = c^2.
// These triples are commonly written as (a, b, c), and a well-known example is (3, 4, 5).

// Write a program that computes the Pythagorean triplet such that a < b < c and a + b + c = 1000.
// */
// fn main() {
//     println!(
//         "The Pythagorean triplet such that a < b < c and a + b + c = 1000 is {:?}",
//         find_pythagorean_triplet()
//     );
// }

// fn find_pythagorean_triplet() -> Vec<i32> {
//     let mut result: Vec<i32> = vec![];
//     for a in 1..=1000 {
//         for b in a + 1..=1000 {
//             let c: i32 = 1000 - a - b;
//             if a * a + b * b == c * c {
//                 result.push(a);
//                 result.push(b);
//                 result.push(c);
//             }
//         }
//     }
//     return result;
// }

// // Problem 6: Write a function that implements the logic,
// // 'You can see the movie if you are 17 or older, or if you are 13 or older and have a parent's permission.'
// // Note: This means that if you 17 or older, you implicitly have permission.

// use std::env;

// fn main() {
//     let args: Vec<String> = env::args().collect();

//     let num: i32 = args[1].parse().unwrap();
//     let flag: bool = args[2].parse().unwrap();

//     println!("Can see a movie {}", can_see_movie(num, flag));
// }

// fn can_see_movie(age: i32, permission: bool) -> bool {
//     (age >= 17) || (age >= 13 && permission)
// }
