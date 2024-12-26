use array_tool::vec::*;
use learn_rust999999911111999::{Category, Customer, Order, Product};

fn main() {
    let product = Product::new(1, "Laptop".to_string(), 1000.0, Category::Electronics);
    let customer = Customer::new(
        1,
        "John Doe".to_string(),
        "vickyvicky@gmail.com".to_string(),
    );
    let order = Order::new(1, product, customer, 10);
    println!("Order Cost: ${:?}", order.total_bill());

    let product1 = Product::new(1, "Laptop".to_string(), 1000.0, Category::Electronics);
    let product2 = Product::new(1, "Book".to_string(), 30.0, Category::Books);
    let product3 = Product::new(1, "Tab".to_string(), 700.0, Category::Electronics);

    let set1 = vec![&product1, &product2];
    let set2 = vec![&product2, &product3];
    let intersection = set1.intersect(set2);
    println!("Intersection: {:?}", intersection);
}
//ciovkntxME0xGeZK3BQLVe4Gf8QZPX0VWNu
