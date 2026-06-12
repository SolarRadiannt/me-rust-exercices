use std::vec::IntoIter;

use crate::exercises::tv2_description::TicketDescription;
use crate::exercises::tv2_title::TicketTitle;

// TODO: Let's start sketching our ticket store!
//  First task: implement `IntoIterator` on `TicketStore` to allow iterating over all the tickets
//  it contains using a `for` loop.
//
// Hint: you shouldn't have to implement the `Iterator` trait in this case.
//   You want to *delegate* the iteration to the `Vec<Ticket>` field in `TicketStore`.
//   Look at the standard library documentation for `Vec` to find the right type
//   to return from `into_iter`.
#[derive(Clone)]
pub struct TicketStore {
    tickets: Vec<Ticket>,
}

impl IntoIterator for TicketStore {
    type Item = Ticket;
    type IntoIter = IntoIter<Ticket>;
    fn into_iter(self) -> Self::IntoIter {
        self.tickets.into_iter()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Ticket {
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status,
}

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}

impl TicketStore {
    pub fn new() -> Self {
        Self {
            tickets: Vec::new(),
        }
    }

    pub fn add_ticket(&mut self, ticket: Ticket) {
        self.tickets.push(ticket);
    }
}

fn ticket_title() -> TicketTitle {
    TicketTitle::try_from("This is a title").unwrap()
}

fn ticket_description() -> TicketDescription {
    TicketDescription::try_from(
        "ths is a description and it is quite lng and it really
        is you cna just read this really if you want proof",
    )
    .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_ticket() {
        let mut store = TicketStore::new();

        let ticket = Ticket {
            title: ticket_title(),
            description: ticket_description(),
            status: Status::ToDo,
        };
        store.add_ticket(ticket);

        let ticket = Ticket {
            title: ticket_title(),
            description: ticket_description(),
            status: Status::InProgress,
        };
        store.add_ticket(ticket);

        let tickets: Vec<_> = store.clone().into_iter().collect();
        assert_eq!(tickets, store.tickets);
    }
}
