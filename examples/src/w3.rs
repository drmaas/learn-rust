//! Week 3 — Structs, enums, pattern matching.

use std::fmt;

#[derive(Debug)]
pub struct Book {
    pub title: String,
    pub pages: u32,
}

impl Book {
    pub fn new(title: &str, pages: u32) -> Book {
        Book {
            title: title.to_string(),
            pages,
        }
    }

    /// `&self` borrows — calling this does NOT destroy the book.
    pub fn reading_minutes(&self, wpm: u32) -> u32 {
        self.pages * 500 / wpm
    }

    pub fn is_long(&self) -> bool {
        self.pages > 400
    }
}

#[derive(Debug)]
pub enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}

/// A `match` on an enum — the compiler requires every variant (exhaustiveness).
pub fn describe(m: &Message) -> String {
    match m {
        Message::Quit => "quit".to_string(),
        Message::Move { x, y } => format!("move to ({x}, {y})"),
        Message::Write(s) => format!("write: {s}"),
        Message::ChangeColor(r, g, b) => format!("color ({r}, {g}, {b})"),
    }
}

/// Handling `Option` three ways: `match`...
pub fn describe_optional_match(v: Option<u32>) -> String {
    match v {
        Some(x) => format!("some {x}"),
        None => "none".to_string(),
    }
}

/// ...`if let`...
pub fn describe_optional_if_let(v: Option<u32>) -> String {
    if let Some(x) = v {
        format!("some {x}")
    } else {
        "none".to_string()
    }
}

/// ...and `let ... else`.
pub fn describe_optional_let_else(v: Option<u32>) -> String {
    let Some(x) = v else {
        return "none".to_string();
    };
    format!("some {x}")
}

/// Patterns: literal, range, wildcard.
pub fn classify_coord(x: i32) -> &'static str {
    match x {
        0 => "origin",
        1..=9 => "small",
        _ => "big",
    }
}

/// Newtypes: `UserId` and `Age` are distinct types — you can't mix them up.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct UserId(u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Age(u8);

#[derive(Debug)]
pub struct User {
    pub id: UserId,
    pub age: Age,
}

pub fn register(user: UserId) -> String {
    format!("registered user {}", user.0)
}

/// Ticket system (Week 3 Day 5 exercise).
#[derive(Debug, Clone, PartialEq)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Ticket {
    pub title: String,
    pub description: String,
    pub status: Status,
}

#[derive(Debug, PartialEq)]
pub enum TicketError {
    EmptyTitle,
    TitleTooLong,
    EmptyDescription,
    DescriptionTooLong,
    InvalidTransition,
}

impl fmt::Display for TicketError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            TicketError::EmptyTitle => "title cannot be empty",
            TicketError::TitleTooLong => "title must be at most 50 chars",
            TicketError::EmptyDescription => "description cannot be empty",
            TicketError::DescriptionTooLong => "description must be at most 500 chars",
            TicketError::InvalidTransition => "invalid status transition",
        };
        write!(f, "{s}")
    }
}

impl Ticket {
    /// Validates; errors are `Result`, not panics.
    pub fn new(title: &str, description: &str, status: Status) -> Result<Ticket, TicketError> {
        if title.is_empty() {
            return Err(TicketError::EmptyTitle);
        }
        if title.chars().count() > 50 {
            return Err(TicketError::TitleTooLong);
        }
        if description.is_empty() {
            return Err(TicketError::EmptyDescription);
        }
        if description.chars().count() > 500 {
            return Err(TicketError::DescriptionTooLong);
        }
        Ok(Ticket {
            title: title.to_string(),
            description: description.to_string(),
            status,
        })
    }

    /// ToDo -> InProgress. Invalid transitions return an error.
    pub fn start(&mut self) -> Result<(), TicketError> {
        match self.status {
            Status::ToDo => {
                self.status = Status::InProgress;
                Ok(())
            }
            _ => Err(TicketError::InvalidTransition),
        }
    }

    /// InProgress -> Done.
    pub fn done(&mut self) -> Result<(), TicketError> {
        match self.status {
            Status::InProgress => {
                self.status = Status::Done;
                Ok(())
            }
            _ => Err(TicketError::InvalidTransition),
        }
    }
}

/// `Display` — human-readable output for a Ticket.
impl fmt::Display for Ticket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{:?}] {}", self.status, self.title)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn book_methods() {
        let b = Book::new("Rust", 300);
        assert_eq!(b.pages, 300);
        assert_eq!(b.reading_minutes(300), 500);
        assert!(!b.is_long());
        let big = Book::new("Big", 500);
        assert!(big.is_long());
    }

    #[test]
    fn message_describe_is_exhaustive() {
        assert_eq!(describe(&Message::Quit), "quit");
        assert_eq!(describe(&Message::Move { x: 1, y: 2 }), "move to (1, 2)");
        assert_eq!(describe(&Message::Write("hi".into())), "write: hi");
        assert_eq!(describe(&Message::ChangeColor(1, 2, 3)), "color (1, 2, 3)");
    }

    #[test]
    fn optional_described_three_ways_agree() {
        assert_eq!(describe_optional_match(Some(7)), "some 7");
        assert_eq!(describe_optional_match(None), "none");
        assert_eq!(describe_optional_if_let(Some(3)), "some 3");
        assert_eq!(describe_optional_if_let(None), "none");
        assert_eq!(describe_optional_let_else(Some(9)), "some 9");
        assert_eq!(describe_optional_let_else(None), "none");
    }

    #[test]
    fn pattern_classification() {
        assert_eq!(classify_coord(0), "origin");
        assert_eq!(classify_coord(5), "small");
        assert_eq!(classify_coord(42), "big");
    }

    #[test]
    fn newtypes_are_distinct() {
        let id = UserId(7);
        let age = Age(30);
        assert_eq!(register(id), "registered user 7");
        // Age and UserId are different types: passing an Age where a UserId
        // is expected is a COMPILE error — that's the point of the newtype.
        assert!(age < Age(40));
        assert_eq!(id, UserId(7));
    }

    #[test]
    fn ticket_validation_rejects_bad_input() {
        assert_eq!(
            Ticket::new("", "desc", Status::ToDo),
            Err(TicketError::EmptyTitle)
        );
        assert_eq!(
            Ticket::new(&"x".repeat(51), "desc", Status::ToDo),
            Err(TicketError::TitleTooLong)
        );
        assert_eq!(
            Ticket::new("t", "", Status::ToDo),
            Err(TicketError::EmptyDescription)
        );
        assert!(Ticket::new("t", "desc", Status::ToDo).is_ok());
    }

    #[test]
    fn ticket_transitions() {
        let mut t = Ticket::new("t", "d", Status::ToDo).unwrap();
        assert_eq!(t.status, Status::ToDo);
        t.start().unwrap();
        assert_eq!(t.status, Status::InProgress);
        assert_eq!(t.start(), Err(TicketError::InvalidTransition));
        t.done().unwrap();
        assert_eq!(t.status, Status::Done);
        assert_eq!(t.done(), Err(TicketError::InvalidTransition));
    }

    #[test]
    fn ticket_display() {
        let t = Ticket::new("Buy milk", "2 litres", Status::ToDo).unwrap();
        assert_eq!(t.to_string(), "[ToDo] Buy milk");
    }
}
