struct Fruit {
    apples: i32,
    bananas: i32,
}

impl Fruit {
    fn new_fruit() -> Fruit {
        Fruit {
            apples: 10,
            bananas: 5,
        }
    }

    fn increase_fruit(&self) -> Fruit {
        Fruit {
            apples: self.apples * 2,
            bananas: self.bananas * 3,
        }
    }

    fn print_fruit(&self) {
        println!(
            "You have {} apples and {} bananas",
            self.apples, self.bananas
        );
    }
}

fn main() {
    Fruit::print_fruit(&Fruit::increase_fruit(&Fruit::new_fruit()));
}
