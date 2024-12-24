struct Car {
    owner: String,
    year: u32,
    fuel_level: f32,
    price: u32,
}

impl Car {
    // Associates functions
    fn monthly_insurance() -> u32 {
        123
    }

    fn selling_price(&self) -> u32 {
        self.price + Car::monthly_insurance()
    }

    fn new(name: String, year: u32) -> Self {
        Self {
            owner: name,
            year: year,
            fuel_level: 0.0,
            price: 0,
        }
    }

    fn display_car_info(&self) {
        println!("Owner: {}", self.owner);
        println!("Year: {}", self.year);
        println!("Fuel Level: {}", self.fuel_level);
        println!("Price: {}", self.price);
    }

    fn refuel(&mut self, gallons: f32) {
        self.fuel_level += gallons;
    }

    fn sell(self) -> Self {
        self
    }
}

fn main() {
    let mut my_car = Car {
        owner: String::from("Alice"),
        year: 2020,
        fuel_level: 50.0,
        price: 20000,
    };

    let extracted_owner: String = my_car.owner.clone(); // fix use .clone() or  &String = &my_car.owner

    // Tuple Structs
    let point_2D = (1, 3);
    let point_3D = (12, 33, 12);

    struct Point_2D(i32, i32);
    struct Point_3D(i32, i32, i32);

    let point = Point_2D(1, 3);
    let point = Point_3D(2, 33, 12);

    //Unit Struct
    struct ABC;

    // using self in 3 diff ways
    my_car.display_car_info();
    my_car.refuel(19.3);
    let new_owner = my_car.sell();

    let new_car = Car::new("Fox".to_string(), 222);
    new_car.display_car_info();
}
