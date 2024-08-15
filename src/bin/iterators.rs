#[derive(Clone, Copy)]
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

    let new_list: Vec<StudentScore> = student_scores
        .iter()
        .map(|stu| StudentScore {
            id: stu.id,
            score: stu.score * 2,
        })
        .collect();

    let passed_student: Vec<StudentScore> = new_list
        .iter()
        .filter(|stu| stu.score > 60)
        .cloned()
        .collect();

    let fav_student = new_list.iter().find(|stu| stu.id == 3);

    let max_marks = new_list.iter().map(|stu| stu.score).max();

    let min_marks = new_list.iter().map(|stu| stu.score).min();

    let count = new_list.iter().count();

    let last = new_list.iter().last();

    let take_use: Vec<_> = new_list.iter().take(5).collect();
}
