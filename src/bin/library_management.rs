enum Genre {
    Fiction,
    NonFiction,
    Science,
    History,
}
struct AvailableDays {
    available_days: i32,
}
enum BookStatus {
    Borrowed(AvailableDays),
    Available,
}
struct Book {
    isbn: String,
    title: String,
    author: String,
    genre: Genre,
    status: BookStatus,
}

impl Book {
    fn new(
        isbn: &str,
        title: &str,
        author: &str,
        genre: Genre,
        status: Option<BookStatus>,
    ) -> Self {
        Book {
            isbn: isbn.to_owned(),
            title: title.to_owned(),
            author: author.to_owned(),
            genre,
            status: status.unwrap_or(BookStatus::Available),
        }
    }

    fn show_book(&self) {
        use BookStatus::*;
        use Genre::*;
        println!(
            "ISBN : {}, Title: {}, Author: {},Genre: {}, Status: {}",
            &self.isbn,
            &self.title,
            &self.author,
            match &self.genre {
                Fiction => "Fiction",
                History => "History",
                NonFiction => "Non-Fiction",
                Science => "Science",
            },
            match &self.status {
                Borrowed(days) => format!("Borrowed for {} days", days.available_days),
                Available => "Available".to_string(),
            }
        )
    }

    fn borrow_book(&mut self, borrow_days: i32) {
        self.status = BookStatus::Borrowed(AvailableDays {
            available_days: borrow_days,
        });
    }

    fn return_book(&mut self) {
        self.status = BookStatus::Available;
    }
}

struct Patron {
    id: i32,
    name: String,
    borrowed_books: Vec<Book>,
}
impl Patron {
    fn new(id: i32, name: String, borrowed_books: Vec<Book>) -> Self {
        Patron {
            id,
            name,
            borrowed_books,
        }
    }

    fn add_book(&mut self) {
        self.borrowed_books.push(Book::new(
            "ISB124548",
            "The Untethered Soul",
            "Rounak Joshi",
            Genre::Fiction,
            Some(BookStatus::Available),
        ));
    }

    fn display_patron(&self) {
        println!("Id: {}, Name: {}", self.id, self.name);
        println!("Borrowed Books");
        for book in &self.borrowed_books {
            book.show_book();
        }
    }
}
struct Library {
    books: Vec<Book>,
    patrons: Vec<Patron>,
}

impl Library {
    fn add_book(
        &mut self,
        isbn: &str,
        title: &str,
        author: &str,
        genre: Genre,
        status: Option<BookStatus>,
    ) {
        self.books
            .push(Book::new(isbn, title, author, genre, status))
    }
    fn add_specific_book(&mut self, book: Book) {
        self.books.push(book)
    }

    fn add_patron(&mut self, patron: Patron) {
        self.add_patron(patron);
    }

    fn display_library(&self) {
        for patron in &self.patrons {
            patron.display_patron();
        }
        for book in &self.books {
            book.show_book();
        }
    }
}

fn main() {
    println!("Library Management System");
}
