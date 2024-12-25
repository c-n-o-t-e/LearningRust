#[derive(Debug)]
enum Days {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

enum TravelType {
    Car(f32),
    Train(f32),
    Plane(f32),
}

impl TravelType {
    fn travel_allowance(&self) -> f32 {
        let allowance = match self {
            TravelType::Car(miles) => miles * 2.0,
            TravelType::Train(miles) => miles * 3.0,
            TravelType::Plane(miles) => miles * 4.0,
        };
        allowance
    }
}

fn main() {
    let today = Days::Sunday;
    print!("{:?}", today);

    let traveller = TravelType::Car(60.0);
    print!("{}", traveller.travel_allowance());

    let hhhh = TravelType::Plane(60.0);
    print!("{}", hhhh.travel_allowance())
}
