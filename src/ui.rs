use inquire::{Confirm, Password, Select, Text};

pub fn easyselect(prompt: &str, choices: Vec<String>) -> String {
	return Select::new(prompt, choices).prompt().unwrap();
}

pub fn easyinq(prompt: &str) -> String {
	return Text::new(prompt).prompt().unwrap();
}

pub fn easypassword(prompt: &str) -> String {
	return Password::new(prompt).prompt().unwrap();
}

pub fn easyconfirm(prompt: &str) -> bool {
	return Confirm::new(prompt).prompt().unwrap();
}
