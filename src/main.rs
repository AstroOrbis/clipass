mod crypto;
mod db;
mod ui;

fn main() {
	let conn: rusqlite::Connection = db::initdb();
	let masterpass: String = ui::easypassword("Enter your Master Password: ");

	if hashy::sha512(masterpass.clone())
		!= db::get_from_meta(&conn, String::from("master_password")).value
	{
		println!("Access denied - quitting program.");
		std::process::exit(0);
	}

	println!("Access granted - welcome to CLIPass!");
	let opts: Vec<String> = vec![
		String::from("Add a new entry"),
		String::from("View an existing entry"),
		String::from("Delete an existing entry"),
		String::from("Change your Master Password"),
		String::from("Quit the program"),
	];


	loop {
		println!("\n");
		match ui::easyselect("What would you like to do?", opts.clone()) {
			choice if choice == opts[0] => {
				let service: String = ui::easyinq("Enter the name of the service:");

				if db::exists_in_table(&conn, service.clone()) {
					println!("Service already exists in database!");
				}

				let pass: String;

				if ui::easyconfirm("Would you like us to generate a password for you? (y/n):") {
					pass = crypto::genpass();
					println!("Your generated password is: {pass}");
				} else {
					pass = ui::easypassword("Enter the password for the service:");
				}

				let note: String = ui::easyinq("Enter a note (leave empty for none):");

				let entry = db::Entry {
					service,
					pass: crypto::encrypt(masterpass.clone(), pass),
					note,
				};
				db::add_to_table(&conn, entry);
				println!("Entry added successfully!");
			}

			choice if choice == opts[1] => {
				let choices = db::list_services(&conn);
				if choices.is_empty() {
					println!("No entries found!");
				}
				let service: String = ui::easyselect("Choose a service", choices);
				let entry = db::get_from_table(&conn, service);
				if entry.note.is_empty() {
					println!(
						"\nService: {}\nPassword: {}",
						entry.service,
						crypto::decrypt(masterpass.clone(), entry.pass),
					);
				} else {
					println!(
						"\nService: {}\nPassword: {}\nNote: {}",
						entry.service,
						crypto::decrypt(masterpass.clone(), entry.pass),
						entry.note
					);
				}
			}

			choice if choice == opts[2] => {
				let choices = db::list_services(&conn);
				if choices.is_empty() {
					println!("No entries found!");
				}
				let service: String = ui::easyselect("Choose a service", choices);
				if ui::easyconfirm("Are you sure you want to delete this entry? (y/n):") {
					db::delete_from_table(&conn, service);
					println!("Entry deleted successfully!");
				} else {
					println!("Entry not deleted.");
					std::process::exit(0);
				}
			}

			choice if choice == opts[3] => {
				let newpass: String = ui::easypassword("Enter your new Master Password: ");
				let hashednewpass = hashy::sha512(newpass);
				db::update_meta(
					&conn,
					db::MetaEntry {
						key: String::from("master_password"),
						value: hashednewpass,
					},
				);
				println!("Master Password changed successfully!");
			}

			choice if choice == opts[4] => {
				println!("Quitting program - see you next time!");
				std::process::exit(0);
			}

			_ => {
				println!("Invalid choice");
				std::process::exit(1);
			}
		}
	}
}
