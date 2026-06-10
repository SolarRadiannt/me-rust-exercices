// TODO: Re-implement `Ticket`'s accessor methods. This time return a `&str` rather than a `&String`.

pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

impl Ticket {
	pub fn new(title: String, description: String, status: String) -> Ticket {
		assert!(!title.is_empty(), "Title cannot be empty!");
		assert!(!description.is_empty(), "Description cannot be empty!");
		
		assert!(title.as_bytes().len() <= 50, "Title cannot be longer than 50 bytes");
		assert!(description.as_bytes().len() <= 500, "Description cannot be longer than 500 bytes");
		
		assert!(status == "To-Do" || status == "Done" || status == "In Progress", "Only `To-Do`, `In Progress`, and `Done` statuses are allowed");
		
		Ticket {
			title,
			description,
			status,
		}
	}

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn status(&self) -> &str {
        &self.status
    }
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::exercises::string_commons::{valid_description, valid_title};
	use std::any::{Any, TypeId};
	
	#[test]
	fn test_type() {
		let ticket = Ticket::new(valid_title(), valid_description(), "To-Do".to_string());
		// Some dark magic to verify that you used the expected return types
		assert_eq!(TypeId::of::<str>(), ticket.title().type_id());
		assert_eq!(TypeId::of::<str>(), ticket.description().type_id());
		assert_eq!(TypeId::of::<str>(), ticket.status().type_id());
	}
}
