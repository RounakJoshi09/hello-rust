use std::f32::consts::E;

struct Employee<State> {
    name: String,
    state: State,
}

impl<State> Employee<State> {
    fn transition<NextState>(self, next_state: NextState) -> Employee<NextState> {
        Employee {
            name: self.name,
            state: next_state,
        }
    }
}

impl Employee<Agreement> {
    fn new(name: &str) -> Self {
        return Self {
            name: name.to_owned(),
            state: Agreement,
        };
    }

    fn read_agreement(self) -> Employee<Signature> {
        return self.transition(Signature);
    }
}

impl Employee<Signature> {
    fn sign(self) -> Employee<Training> {
        return self.transition(Training);
    }
}

impl Employee<Training> {
    fn train(
        self,
        score: usize,
    ) -> Result<(Employee<OnboardingCompleted>, usize), (Employee<OnboardingFailed>, usize)> {
        if score > 7 {
            return Ok((self.transition(OnboardingCompleted), score));
        } else {
            return Err((self.transition(OnboardingFailed), score));
        }
    }
}

struct Agreement;
struct Signature;
struct Training;
struct OnboardingCompleted;
struct OnboardingFailed;

fn main() {
    let employee = Employee::new("Rounak");
    let result = employee.read_agreement().sign().train(10);

    match result {
        Ok(emp) => println!("Onboarding Completed"),
        Err(emp) => println!("Onboarding Failed : score {:?}", emp.1),
    }
}
