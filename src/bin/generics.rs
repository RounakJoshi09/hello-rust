#[derive(Debug)]
enum ServicePriority {
    High,
    Standard,
}

trait Priority {
    fn get_priority(&self) -> ServicePriority;
}

#[derive(Debug)]
struct ImportantGuest;
impl Priority for ImportantGuest {
    fn get_priority(&self) -> ServicePriority {
        ServicePriority::High
    }
}

#[derive(Debug)]
struct Guest;
impl Priority for Guest {
    fn get_priority(&self) -> ServicePriority {
        ServicePriority::Standard
    }
}

// fn print_guest_details<T: Priority>(guest: T) {
//     println!("{}", guest.get_priority());
// }

fn print_guest_details<T>(guest: T)
where
    T: Priority,
{
    println!("{:?}", guest.get_priority());
}

fn main() {
    print_guest_details(ImportantGuest);
    print_guest_details(Guest);
    print_guest_details(ImportantGuest)
}
