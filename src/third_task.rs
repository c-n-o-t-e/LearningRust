fn main() {
    // This question involves writing code to analyze the production of an assembly line in a car factory. 
    // The assembly line has different speeds, ranging from 0 (off) to 10 (maximum). At the lowest speed of 1, the assembly line produces a total of 221 cars per hour. 
    // The production rate increases linearly with the speed, meaning that a speed of 4 produces 4 * 221 = 884 cars per hour.
    // However, higher speeds increase the likelihood of producing faulty cars that need to be discarded. 
    // The success rate depends on the speed, as shown in the table below:
    // · Speeds 1 to 4: 100% success rate.
    // · Speeds 5 to 8: 90% success rate.
    // · Speeds 9 and 10: 77% success rate.

    // You need to write two functions:
    // 1. The first function, total_production(), calculates the total number of cars successfully produced without faults within a specified time given in hours. 
    //    The function takes the number of hours and speed as input and returns the number of cars successfully produced.
    // 2. The second function, cars_produced_per_minute(), calculates the number of cars successfully produced per minute. 
    //    The function takes the number of hours and speed as input and returns the number of cars produced per minute.

    // Write the code for both functions based on the provided specifications.

    let mut number = String::new();
    std::io::stdin()
        .read_line(&mut number)
        .expect("Failed to read input.");

    let values: Vec<u128> = number.trim().split(',').map(|s| s.parse().unwrap()).collect();
    let (number_of_cars, number_of_hours) = total_production(values[0],values[1]);
    let cars_produced_per_minute = cars_produced_per_minute(values[0],values[1]);

    println!("The total number of cars successfully produced without faults within {} hours is {}", number_of_hours, number_of_cars);
    println!("The total number of cars successfully produced per minute within {} hours is {}", number_of_hours, cars_produced_per_minute);
}

fn get_speed_success_rate(speed_rate: u128) -> u128 {
    let speed_success_rate: u128 = match speed_rate {
        1..=4 => 100,
        5..=8 => 90,
        9..=10 => 77,
        _ => 0
    }; 
    return speed_success_rate;
}

fn total_production(number_of_hours: u128, speed: u128) -> (u128, u128) {
    let mut good_cars:u128 = 0;
    let number_of_car_produce_within_an_hour: u128 = 221;
    let success_rate: u128 = get_speed_success_rate(speed);

    if success_rate == 100{
        good_cars = number_of_car_produce_within_an_hour * number_of_hours;
    }
    return (good_cars, number_of_hours);
} 

fn cars_produced_per_minute(number_of_hours: u128, speed: u128) -> f64 {
    let mut good_cars:f64 = 0.0;
    let number_of_car_produce_per_minute: f64 = 221.0 / 60.0;
    let success_rate: u128 = get_speed_success_rate(speed);

    if success_rate == 100{
        good_cars = number_of_car_produce_per_minute * (number_of_hours as f64);
    }
    return good_cars;
} 