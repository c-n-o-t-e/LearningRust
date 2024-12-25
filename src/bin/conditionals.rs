fn main() {
    let num: i32 = 40;

    if num > 30 {
        println!("The number is greater than 30");
    } else {
        println!("The number is less than 30");
    }

    let marks: i32 = 85;
    let mut grade: char = 'F';

    // using if, else, and if else
    grade = if marks >= 90 {
        'A'
    } else if marks >= 80 {
        'B'
    } else if marks >= 70 {
        'C'
    } else if marks >= 60 {
        'D'
    } else {
        'F'
    };

    // using match
    grade = match marks {
        90..=100 => 'A',
        80..=89 => 'B',
        70..=79 => 'C',
        60..=69 => 'D',
        _ => 'F',
    };

    print!("The grade is: {}", grade);
}
