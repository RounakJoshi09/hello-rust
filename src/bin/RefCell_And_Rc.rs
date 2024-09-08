// Practice for Smart Pointers and Interior Mutability
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum MenuItem {
    Drink,
    Salad,
}

#[derive(Debug)]
struct ItemOrder {
    item: MenuItem,
    quantity: i32,
}

#[derive(Debug)]
struct TableOrder {
    items: Vec<ItemOrder>,
}

fn new_table_order() -> TableOrder {
    TableOrder {
        items: vec![ItemOrder {
            item: MenuItem::Salad,
            quantity: 2,
        }],
    }
}

type Order = Rc<RefCell<Vec<TableOrder>>>;

#[derive(Debug)]
struct Waiter(Order);
#[derive(Debug)]
struct Chef(Order);
#[derive(Debug)]
struct Accounts(Order);

fn main() {
    let orders = Rc::new(RefCell::new(vec![]));

    let chef = Chef(Rc::clone(&orders));

    let waiter = Waiter(Rc::clone(&orders));

    let accounts = Accounts(Rc::clone(&orders));
    {
        let mut chef_mut = chef.0.borrow_mut();
        chef_mut.push(new_table_order());
    }

    dbg!(chef.0.borrow());
    drop(chef);
    dbg!(waiter.0.borrow());
    drop(waiter);
    dbg!(accounts);
}
