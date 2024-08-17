use thiserror::Error;

#[derive(Debug, Error)]
enum ProgramError {
    #[error("This is Menu Error")]
    Menu(#[from] MenuError),
    #[error("This is Math Error")]
    Math(#[from] MathError),
}

#[derive(Debug, Error)]
enum MenuError {
    #[error("menu item not found")]
    NotFound,
}

#[derive(Debug, Error)]
enum MathError {
    #[error("divide by zero error")]
    DivideByZero,
}

fn pick_menu(choice: &str) -> Result<i32, MenuError> {
    match choice {
        "1" => Ok(1),
        "2" => Ok(2),
        "3" => Ok(3),
        _ => Err(MenuError::NotFound),
    }
}

fn divide(a: i32, b: i32) -> Result<i32, MathError> {
    if b != 0 {
        Ok(a / b)
    } else {
        Err(MathError::DivideByZero)
    }
}

fn run(step: i32) -> Result<(), ProgramError> {
    if step == 1 {
        pick_menu("4")?;
    } else if step == 2 {
        divide(1, 0)?;
    }
    Ok(())
}

fn main() {
    println!("{:?}", run(1));
    println!("{:?}", run(2));
}
