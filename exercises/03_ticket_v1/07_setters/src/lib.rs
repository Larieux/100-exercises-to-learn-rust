// TODO: Add &mut-setters to the `Ticket` struct for each of its fields.
//   Make sure to enforce the same validation rules you have in `Ticket::new`!
//   Even better, extract that logic and reuse it in both places. You can use
//   private functions or private static methods for that.

pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

impl Ticket {
    pub fn new(title: String, description: String, status: String) -> Ticket {
		Self::check_title_borrow(&title);
		Self::check_description_borrow(&description);
		Self::check_status_borrow(&status);

        Ticket {
            title,
            description,
            status,
        }
    }

	pub fn set_title(&mut self, title: String) {
		Self::check_title_borrow(&title);
		self.title = title;
	}

	pub fn set_description(&mut self, description: String) {
		Self::check_description_borrow(&description);
		self.description = description;
	}

	pub fn set_status(&mut self, status: String) {
		Self::check_status_borrow(&status);
		self.status = status;
	}

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn description(&self) -> &String {
        &self.description
    }

    pub fn status(&self) -> &String {
        &self.status
    }

	fn check_title_borrow(title: &String) {
		assert !(!title.is_empty(), "Title cannot be empty");
		assert !(title.len() < 50, "Title cannot be longer than 50 bytes");
	}

	fn check_description_borrow(description: &String) {
		assert !(!description.is_empty(), "Description cannot be empty");
		assert !(description.len() < 500, "Description cannot be longer than 500 bytes");
	}
	
	fn check_status_borrow(status: &String) {
		assert !(match status.as_str() {
			"To-Do" => true,
			"In Progress" => true,
			"Done" => true,
			_ => false
		}, "Only `To-Do`, `In Progress`, and `Done` statuses are allowed");
	}

	// fn check_title_own(title: String) -> String {
	// 	assert !(!title.is_empty(), "Title cannot be empty");
	// 	assert !(title.len() < 50, "Title cannot be longer than 50 bytes");
	// 	title
	// }

	// fn check_description_own(description: String) -> String {
	// 	assert !(!description.is_empty(), "description cannot be empty");
	// 	assert !(description.len() < 500, "description cannot be longer than 50 bytes");
	// 	description
	// }

	// fn check_status_own(status: String) -> String {
	// 	assert !(match status.as_str() {
	// 		"To-Do" => true,
	// 		"In Progress" => true,
	// 		"Done" => true,
	// 		_ => false
	// 	}, "Only `To-Do`, `In Progress`, and `Done` statuses are allowed");
	// 	status
	// }

}

#[cfg(test)]
mod tests {
    use super::Ticket;
    use common::{overly_long_description, overly_long_title, valid_description, valid_title};

    #[test]
    fn works() {
        let mut ticket = Ticket::new("A title".into(), "A description".into(), "To-Do".into());
        ticket.set_title("A new title".into());
        ticket.set_description("A new description".into());
        ticket.set_status("Done".into());

        assert_eq!(ticket.title(), "A new title");
        assert_eq!(ticket.description(), "A new description");
        assert_eq!(ticket.status(), "Done");
    }

    #[test]
    #[should_panic(expected = "Title cannot be empty")]
    fn title_cannot_be_empty() {
        Ticket::new(valid_title(), valid_description(), "To-Do".into()).set_title("".into());
    }

    #[test]
    #[should_panic(expected = "Description cannot be empty")]
    fn description_cannot_be_empty() {
        Ticket::new(valid_title(), valid_description(), "To-Do".into()).set_description("".into());
    }

    #[test]
    #[should_panic(expected = "Title cannot be longer than 50 bytes")]
    fn title_cannot_be_longer_than_fifty_chars() {
        Ticket::new(valid_title(), valid_description(), "To-Do".into())
            .set_title(overly_long_title())
    }

    #[test]
    #[should_panic(expected = "Description cannot be longer than 500 bytes")]
    fn description_cannot_be_longer_than_500_chars() {
        Ticket::new(valid_title(), valid_description(), "To-Do".into())
            .set_description(overly_long_description())
    }

    #[test]
    #[should_panic(expected = "Only `To-Do`, `In Progress`, and `Done` statuses are allowed")]
    fn status_must_be_valid() {
        Ticket::new(valid_title(), valid_description(), "To-Do".into()).set_status("Funny".into());
    }
}
