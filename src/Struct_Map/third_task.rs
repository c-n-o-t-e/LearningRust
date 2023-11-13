#[derive(Debug)]
enum ItemType {Book, Magazine}

#[derive(Debug)]
 struct Item {
    id: u32,
    title: String,
    year: u32,
    type_of_book: ItemType
}

impl Item {
    fn display_item_info(self) -> Item {
        self
    }
}

fn main() {
    let item = Item {
        id: 1,
        title: String::from("Victor"),
        year: 40,
        type_of_book: ItemType::Book,
    };

    println!("The structure values are {:?}", item.display_item_info());
}

