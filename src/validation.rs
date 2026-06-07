
pub mod ticket {
	pub struct Ticket {
		title: String,
		description: String,
		status: String,
	}
	
	impl Ticket {
		pub fn new(title: String, description: String, status: String) -> Self {
			assert!(!title.is_empty(), "Title cannot be empty!");
			assert!(!description.is_empty(), "Description cannot be empty!");
			
			assert!(title.as_bytes().len() <= 50, "Title cannot be longer than 50 bytes");
			assert!(description.as_bytes().len() <= 500, "Description cannot be longer than 500 bytes");
			
			assert!(status == "To-Do" || status == "Done" || status == "In Progress", "Only `To-Do`, `In Progress`, and `Done` statuses are allowed");
			
			Self {
				title,
				description,
				status,
			}
		}
		
		pub fn description(&self) -> &String {
			&self.description
		}

		pub fn title(&self) -> &String {
			&self.title
		}

		pub fn status(&self) -> &String {
			&self.status
		}
		pub fn set_title(&mut self, title: String) -> &mut Self {
			self.title = title;
			self
		}
		pub fn set_description(&mut self, description: String) -> &mut Self {
			self.description = description;
			self
		}
		pub fn set_status(&mut self, status: String) -> &mut Self {
			self.status = status;
			self
		}
		
	}
}

#[cfg(test)]

mod tests {
	use super::ticket::Ticket;
	use crate::string_commons::{
		overly_long_description,
		overly_long_title,
		valid_description,
		valid_title,
	};
    
    #[test]
    #[should_panic(expected = "Title cannot be empty")]
    fn title_cannot_be_empty() {
        Ticket::new("".into(), valid_description(), "To-Do".into());
    }

    #[test]
    #[should_panic(expected = "Description cannot be empty")]
    fn description_cannot_be_empty() {
        Ticket::new(valid_title(), "".into(), "To-Do".into());
    }

    #[test]
    #[should_panic(expected = "Title cannot be longer than 50 bytes")]
    fn title_cannot_be_longer_than_fifty_chars() {
        Ticket::new(overly_long_title(), valid_description(), "To-Do".into());
    }

    #[test]
    #[should_panic(expected = "Description cannot be longer than 500 bytes")]
    fn description_cannot_be_longer_than_500_chars() {
        Ticket::new(valid_title(), overly_long_description(), "To-Do".into());
    }

    #[test]
    #[should_panic(expected = "Only `To-Do`, `In Progress`, and `Done` statuses are allowed")]
    fn status_must_be_valid() {
        Ticket::new(valid_title(), valid_description(), "Funny".into());
    }

    #[test]
    fn done_is_allowed() {
        Ticket::new(valid_title(), valid_description(), "Done".into());
    }

    #[test]
    fn in_progress_is_allowed() {
        Ticket::new(valid_title(), valid_description(), "In Progress".into());
    }

    #[test]
    fn getter_test() {
        let ticket = Ticket::new("A title".into(), "A description".into(), "To-Do".into());
        // If you change the signatures as requested, this should compile:
        // we can call these methods one after the other because they borrow `self`
        // rather than taking ownership of it.
        assert_eq!(ticket.title(), "A title");
        assert_eq!(ticket.description(), "A description");
        assert_eq!(ticket.status(), "To-Do");
    }

	#[test]
	fn setter_test() {
		let mut ticket = Ticket::new("A title".into(), "A description".into(), "To-Do".into());
		ticket.set_title("A new title".into());
		ticket.set_description("A new description".into());
		ticket.set_status("Done".into());
		
		
		assert_eq!(ticket.title(), "A new title");
		assert_eq!(ticket.description(), "A new description");
		assert_eq!(ticket.status(), "Done");
	}
}
