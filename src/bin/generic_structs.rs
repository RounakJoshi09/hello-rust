enum FrontendTechStack {
    React,
    Next,
}
enum FullStackTechStack {
    React,
    Next,
    Node,
    CSharp,
}

trait DevelopFrontend {
    fn can_work_on_react(&self) -> bool;
    fn can_work_on_next(&self) -> bool;
}

struct FrontendEngineer {
    name: String,
    skill: FrontendTechStack,
}
struct FullStackEngineer {
    name: String,
    skill: Vec<FullStackTechStack>,
}

impl DevelopFrontend for FrontendEngineer {
    fn can_work_on_next(&self) -> bool {
        match self.skill {
            FrontendTechStack::Next => true,
            _ => false,
        }
    }
    fn can_work_on_react(&self) -> bool {
        match self.skill {
            FrontendTechStack::React => true,
            _ => false,
        }
    }
}

impl DevelopFrontend for FullStackEngineer {
    fn can_work_on_next(&self) -> bool {
        return self
            .skill
            .iter()
            .find(|tech_stack| match *tech_stack {
                FullStackTechStack::Next => true,
                _ => false,
            })
            .is_some();
    }
    fn can_work_on_react(&self) -> bool {
        return self
            .skill
            .iter()
            .find(|tech_stack| match *tech_stack {
                FullStackTechStack::React => true,
                _ => false,
            })
            .is_some();
    }
}

struct UIEmployee<T: DevelopFrontend> {
    employee: T,
}

fn main() {
    let employee1 = FullStackEngineer {
        name: "Rounak".to_owned(),
        skill: vec![
            FullStackTechStack::React,
            FullStackTechStack::Next,
            FullStackTechStack::Node,
        ],
    };

    let employee2 = FrontendEngineer {
        name: "Joshi".to_owned(),
        skill: FrontendTechStack::React,
    };

    let employee3 = FullStackEngineer {
        name: "Ram".to_owned(),
        skill: vec![FullStackTechStack::Node],
    };

    let ui_employee_1 = UIEmployee {
        employee: employee1,
    };
    let ui_employee_2 = UIEmployee {
        employee: employee2,
    };
    let ui_employee_3: UIEmployee<FullStackEngineer> = UIEmployee {
        employee: employee3,
    };

    let select_employee_1 =
        ui_employee_1.employee.can_work_on_next() || ui_employee_1.employee.can_work_on_react();
    let select_employee_2 =
        ui_employee_2.employee.can_work_on_next() || ui_employee_2.employee.can_work_on_react();
    let select_employee_3 =
        ui_employee_3.employee.can_work_on_next() || ui_employee_3.employee.can_work_on_react();

    println!(
        " Employee : {:?} is selected-> :{:?}",
        ui_employee_1.employee.name, select_employee_1
    );
    println!(
        " Employee : {:?} is selected-> :{:?}",
        ui_employee_2.employee.name, select_employee_2
    );
    println!(
        " Employee : {:?} is selected-> :{:?}",
        ui_employee_3.employee.name, select_employee_3
    );
}
