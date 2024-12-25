use std::primitive;

#[derive(Debug)]

struct Student {
    name: String,
    grade: Option<u32>,
}

fn get_grade(student_name: &String, student_db: &Vec<Student>) -> Option<u32> {
    for student in student_db {
        if &student.name == student_name {
            return student.grade;
        }
    }
    None
}

// better way to handle things
fn check_student(student_name: &String, student_db: &Vec<Student>) -> Result<Option<u32>, String> {
    for student in student_db {
        if &student.name == student_name {
            return Ok(student.grade);
        }
    }
    Err(String::from("Student not found"))
}

fn main() {
    let student_db = vec![
        Student {
            name: String::from("gant"),
            grade: Some(76),
        },
        Student {
            name: String::from("uoy"),
            grade: Some(34),
        },
        Student {
            name: String::from("rubby"),
            grade: Some(88),
        },
    ];

    let student_name: String = String::from("ganyt");
    let student_grade = get_grade(&student_name, &student_db);
    print!("{:?}", student_grade);

    let student_status = check_student(&student_name, &student_db);

    match student_status {
        Ok(option_grade) => {
            if let Some(grade) = option_grade {
                print!("Grade is: {grade}");
            }
        }
        Err(error_msg) => print!("{error_msg}"),
    }
}
