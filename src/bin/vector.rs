struct StudentScore {
    score: i32,
    id: i32,
}
fn main() {
    let student_scores = vec![
        StudentScore { id: 1, score: 30 },
        StudentScore { id: 2, score: 50 },
        StudentScore { id: 3, score: 60 },
    ];

    let mut class_score = Vec::new();
    class_score.push(StudentScore { id: 30, score: 100 });
    class_score.push(StudentScore { id: 40, score: 120 });
    class_score.push(StudentScore { id: 50, score: 150 });

    class_score.pop();

    for score in student_scores {
        println!("id = {:?}, score = {:?}", score.id, score.score);
    }

    println!("Class Score {:?}", class_score.len());

    for score in class_score {
        println!("id = {:?}, score = {:?}", score.id, score.score);
    }
}
