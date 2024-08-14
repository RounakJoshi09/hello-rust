// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info

struct TicketInfo {
    name: String,
    price: f64,
}

enum Ticket {
    Backstage(TicketInfo),
    Vip(TicketInfo),
    Standard(TicketInfo),
}

impl Ticket {
    fn print_ticket_info(&self) {
        match self {
            Ticket::Backstage(ticket) => {
                println!("Backstage Name : {}, Price: {}", ticket.name, ticket.price)
            }
            Ticket::Standard(ticket) => {
                println!("Standard Name : {}, Price: {}", ticket.name, ticket.price)
            }
            Ticket::Vip(ticket) => {
                println!("VIP Name : {}, Price: {}", ticket.name, ticket.price)
            }
        }
    }
}

fn main() {
    let tickets: Vec<Ticket> = vec![
        Ticket::Backstage(TicketInfo {
            name: "Rounak Joshi".to_owned(),
            price: 50.0,
        }),
        Ticket::Vip(TicketInfo {
            name: "Pundrikaksh".to_owned(),
            price: 25.5,
        }),
        Ticket::Standard(TicketInfo {
            name: "Ram Krisna".to_owned(),
            price: 100.5,
        }),
    ];

    for ticket in tickets {
        ticket.print_ticket_info();
    }
}
