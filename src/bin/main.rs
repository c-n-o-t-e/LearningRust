use learn_rust::{Category, Customer, Order, Product};

fn main() {
    let product = Product::new(1, "Laptop".to_string(), 1000.0, Category::Electronics);
    let customer = Customer::new(
        1,
        "John Doe".to_string(),
        "vickyvicky@gmail.com".to_string(),
    );
    let order = Order::new(1, product, customer, 10);
    println!("Order Cost: ${:?}", order.total_bill());
}
