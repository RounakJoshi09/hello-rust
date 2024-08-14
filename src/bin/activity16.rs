// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

struct LockerDetails {
    student_name: String,
    locker_assignment: Option<i32>,
}

impl LockerDetails {
    fn print_locker_details(&self) {
        println!("Name : {}", self.student_name);

        match self.locker_assignment {
            Some(num) => println!("Number {}", num),
            None => print!("No Locker Assignment Number"),
        }
    }
}

fn main() {
    let student_details = vec![
        LockerDetails {
            locker_assignment: Some(50),
            student_name: "Rounak".to_owned(),
        },
        LockerDetails {
            locker_assignment: Some(20),
            student_name: "John".to_owned(),
        },
        LockerDetails {
            locker_assignment: None,
            student_name: "Yash".to_owned(),
        },
    ];

    for student in student_details {
        student.print_locker_details();
    }
}
