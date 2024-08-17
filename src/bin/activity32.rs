// Topic: Lifetimes & Structures
//
// Requirements:
// * Display just the names and titles of persons from the mock-data.csv file
// * The names & titles must be stored in a struct separately from the mock
//   data for potential later usage
// * None of the mock data may be duplicated in memory
//
// Notes:
// * The mock data has already been loaded with the include_str! macro, so all functionality
//   must be implemented using references/borrows

const MOCK_DATA: &'static str = include_str!("mock-data.csv");

struct Person<'a> {
    name: &'a str,
    title: &'a str,
}

impl<'a> Person<'a> {
    fn new(name: &'a str, title: &'a str) -> Self {
        return Self { name, title };
    }
}

fn main() {
    let data_array: Vec<&str> = MOCK_DATA.split('\n').skip(1).collect();

    let person_data: Vec<Option<Person>> = data_array
        .iter()
        .map(|data| -> Option<Person> {
            let details: Vec<&str> = data.split(',').collect();

            if details.len() < 5 {
                return None;
            }

            return Some(Person::new(details[1], details[4]));
        })
        .collect();

    person_data.iter().for_each(|person| match person {
        Some(person) => println!("Name : {:?}, Title: {:?}", person.name, person.title),
        None => (),
    });
}
