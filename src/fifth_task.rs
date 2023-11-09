fn main() {
    // A Pythagorean triple consists of three positive integers a, b, and c, satisfying the condition a^2 + b^2 = c^2. 
    // These triples are commonly written as (a, b, c), and a well-known example is (3, 4, 5).
    // Write a program that computes the Pythagorean triplet such that a < b < c and a + b + c = 1000

    println!("The Pythagorean triplet such that a < b < c and a + b + c = 1000 is {:?}", find_pythagorean_triplet());
}

fn find_pythagorean_triplet() -> Vec<i32> {
    let mut result: Vec<i32> = vec![];
    for a in 1..=1000  {
        for b in a + 1..=1000  {
            let c: i32 = 1000 - a - b;
            if a*a + b*b == c*c {
                result.push(a);
                result.push(b);
                result.push(c);
            }
        }
    }
    return  result;
}