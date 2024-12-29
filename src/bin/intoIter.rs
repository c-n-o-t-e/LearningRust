// -------------------------------------------
// 		IntoIterator
// -------------------------------------------
/*
trait IntoIterator {
    type Item;
    type IntoIter: Iterator;
    fn into_iter(self) -> Self::IntoIter;
}
*/

#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    genre: String,
}

// struct BookIterator {
//     properties: Vec<String>,
// }

// impl Iterator for BookIterator {
//     type Item = String;

//     fn next(&mut self) -> Option<Self::Item> {
//         if !self.properties.is_empty() {
//             Some(self.properties.remove(0))
//         } else {
//             None
//         }
//     }
// }

impl IntoIterator for Book {
    type Item = Book;
    // type IntoIter = BookIterator;

    // fn into_iter(self) -> Self::IntoIter {
    //     BookIterator {
    //         properties: vec![self.title, self.author, self.genre],
    //     }
    // }

    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        vec![self].into_iter()
    }
}

fn main() {
    let book = Book {
        title: "Digital Image Processing".to_string(),
        author: "Gonzales".to_string(),
        genre: "Science Book".to_string(),
    };

    let book1 = Book {
        title: "Digital Image Processing".to_string(),
        author: "Gonzales".to_string(),
        genre: "Science Book".to_string(),
    };

    let books = vec![&book, &book1];

    let mut book_iterator = books.into_iter();

    println!("{:?}", book_iterator.next());
    println!("{:?}", book_iterator.next());
    println!("{:?}", book_iterator.next());
    println!("{:?}", book_iterator.next());

    // for book_info in book_iterator {
    //     println!("{:?}", book_info);
    // }
}
