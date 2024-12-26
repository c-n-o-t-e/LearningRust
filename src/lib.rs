//! # Online Store
//! This is a library for an online store that allows you to create products, customers and orders.

pub use customer::Customer;
pub use order::Order;
pub use product::{Category, Product};

mod product {
    pub use category::Category;
    #[derive(PartialEq, Debug)]

    /// A struct representing a product
    pub struct Product {
        pub id: u64,
        name: String,
        price: f64,
        category: Category,
    }

    impl Product {
        /// #Test Example
        /// ```
        /// use learn_rust999999911111999::{Category, Product};    
        /// let some_product = Product::new(1, "Laptop".to_string(), 1000.0, Category::Electronics);
        /// assert_eq!(some_product.id, 1);
        /// ```
        pub fn new(id: u64, name: String, price: f64, category: Category) -> Product {
            Product {
                id,
                name,
                price,
                category,
            }
        }

        fn calculate_tax(&self) -> f64 {
            self.price * 0.1
        }

        pub fn product_price(&self) -> f64 {
            self.price + self.calculate_tax()
        }
    }

    mod category {
        #[derive(PartialEq, Debug)]
        /// A module representing the category of a product
        pub enum Category {
            Electronics,
            Clothing,
            Books,
        }
    }
}

mod customer {
    pub struct Customer {
        id: u64,
        name: String,
        email: String,
    }

    impl Customer {
        pub fn new(id: u64, name: String, email: String) -> Customer {
            Customer { id, name, email }
        }
    }
}

mod order {
    use crate::customer::Customer;
    use crate::product::Product;
    pub struct Order {
        id: u64,
        product: Product,
        customer: Customer,
        quantity: u32,
    }

    impl Order {
        pub fn new(id: u64, product: Product, customer: Customer, quantity: u32) -> Order {
            Order {
                id,
                product,
                customer,
                quantity,
            }
        }
        fn calculate_discount(&self) -> f64 {
            if self.quantity > 5 {
                0.1
            } else {
                0.0
            }
        }

        pub fn total_bill(&self) -> f64 {
            let discount = self.calculate_discount();
            let total_before_discount = self.product.product_price() * self.quantity as f64;
            total_before_discount - (total_before_discount * discount)
        }
    }
}
