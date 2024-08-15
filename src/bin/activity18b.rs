// Topic: Result & the question mark operator
//
// Requirements:
// * Determine if an employee can access a building using a digital keycard
// * Employees that can access the building are:
//   * Maintenance crews
//   * Marketing department employees
//   * Managers
// * Other employees that work at the company are:
//   * Line supervisors
//   * Kitchen staff
//   * Assembly technicians
// * Ensure that terminated employees cannot access the building
//   regardless of their position
//
// Notes:
// * Use an enum to represent all types of employees
// * Use a struct to store the employee type and whether they are
//   still employed
// * Use a function that returns a Result to determine if the employee
//   may enter the building
// * Print whether the employee may access the building
//   * Must use a function that utilizes the question mark operator to do this

enum EmployeeType {
    MaintainenceCrew,
    MarketingDepartment,
    Managers,
    LineSupervisors,
    KitchenStaff,
    AssemblyTechnician,
}

struct Employee {
    id: i32,
    employee_type: (EmployeeType),
    employed: bool,
}

impl Employee {
    fn can_access_building(&self) -> Result<bool, String> {
        match self.employee_type {
            EmployeeType::MaintainenceCrew => return Ok(self.employed),
            EmployeeType::MarketingDepartment => return Ok(self.employed),
            EmployeeType::Managers => return Ok(self.employed),
            _ => return Err("Cannot Access the Building".to_string()),
        }
    }

    fn check_building_access(&self) -> Result<(), String> {
        let result = self.can_access_building()?;

        if result == true {
            println!("Can Access the Building")
        } else {
            println!("Cannot access the building as you are terminated");
        }
        return Ok(());
    }

    fn new(id: i32, employee_type: EmployeeType, is_employed: bool) -> Self {
        return Self {
            id: id,
            employee_type: employee_type,
            employed: is_employed,
        };
    }
}

fn main() {
    let employee_list = vec![
        Employee::new(10, EmployeeType::AssemblyTechnician, true),
        Employee::new(12, EmployeeType::MaintainenceCrew, false),
        Employee::new(15, EmployeeType::MaintainenceCrew, true),
        Employee::new(16, EmployeeType::LineSupervisors, true),
        Employee::new(16, EmployeeType::Managers, false),
    ];

    for employee in employee_list {
        let err = employee.check_building_access();
        match err {
            Err(str) => println!("{str}"),
            _ => (),
        }
    }
}
