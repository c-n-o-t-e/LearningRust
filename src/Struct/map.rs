use std::collections::HashMap;

struct Student {
    name: String,
    age: i32,
    grade: String,
}

impl Student {
    fn add_student(
        student_database: &mut HashMap<i32, Student>,
        id: i32,
        name: String,
        age: i32,
        grade: String,
    ) {
        student_database.entry(id).or_insert(Student{
            name: name,
            age: age,
            grade: grade,  
        });
    }
}

fn main() {
    let mut student_database: HashMap<i32, Student> = HashMap::new();
    Student::add_student(
        &mut student_database,
        1,
        String::from("John"),
        17,
        String::from("Grade 11"),
    );

    Student::add_student(
        &mut student_database,
        2,
        String::from("Sarah"),
        16,
        String::from("Grade 10"),
    );

    // Printing the student database

    for (id, student) in &student_database {
        println!("Student ID: {}", id);
        println!("Name: {}", student.name);
        println!("Age: {}", student.age);
        println!("Grade: {}", student.grade);
        println!("------------------");
    }
}

