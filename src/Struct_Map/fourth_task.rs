use std::collections::HashMap;

#[derive(Debug)]
struct Student {
    id: i32,
    name: String,
    grade: char
}

struct StudentManager {
    map: HashMap<i32, Student>,
}

impl StudentManager {
    fn new() -> Self {
        Self{map: HashMap::new()}
    }

    fn  add_student(&mut self, student: Student) -> Result<(), String> {
        if self.map.contains_key(&student.id) {
            Err(String::from("student's ID already exists"))
        }else {
            self.map.insert(student.id,student);
            Ok(())
        }
    }

    fn get_student(&self, id: i32) -> Option<&Student> {
        if self.map.contains_key(&id) {
            self.map.get(&id)
        }else {
            None
        }
    }
}

fn main() {
    let mut student_database = StudentManager::new();

    student_database.add_student( Student{
        id: 1,
        name: String::from("Victor"),
        grade: 'A'
    });


    println!("{:?}", student_database.get_student(1));
}