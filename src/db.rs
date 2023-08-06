use super::ui;
use hashy::sha512;

use rusqlite::Connection;
use std::path::Path;
use touch::{dir, file};

pub struct Entry {
	pub service: String,
	pub pass: String,
	pub note: String,
}

pub struct MetaEntry {
	pub key: String,
	pub value: String,
}

pub fn initdb() -> Connection {
	if !Path::new(getdbfile().as_str()).exists() {
		println!("Creating database files...");
		let basedir: String = format!(
			"{}{}{}",
			dirs::home_dir().unwrap().display(),
			std::path::MAIN_SEPARATOR,
			".clipass"
		);
		let dbfile: String = format!("{}{}{}", basedir, std::path::MAIN_SEPARATOR, "clipass.db");

		if !Path::new(basedir.as_str()).exists() {
			println!("Creating directory ~/.clipass...");

			match dir::create(basedir.as_str()) {
				Ok(_) => println!("Created directory ~/.clipass"),
				Err(_) => {
					panic!("Failed to create database files. Do you have the right permissions?");
				}
			};
		}

		if !Path::new(dbfile.as_str()).exists() {
			println!("Creating database file...");

			match file::create(dbfile.as_str(), false) {
				Ok(_) => println!("Created file ~/.clipass/clipass.db"),
				Err(_) => {
					println!("Failed to create database files. Do you have the right permissions?");
				}
			}
		}

		Connection::open(getdbfile())
			.unwrap()
			.execute(
				"CREATE TABLE IF NOT EXISTS passwords (
					service TEXT PRIMARY KEY,
					pass TEXT NOT NULL,
					note TEXT NOT NULL
				);",
				[],
			)
			.unwrap();

		Connection::open(getdbfile())
			.unwrap()
			.execute(
				"CREATE TABLE IF NOT EXISTS meta (
			key TEXT PRIMARY KEY,
			value TEXT NOT NULL
		)",
				[],
			)
			.unwrap();

		println!("Created tables.");

		println!("\nIt is now time to choose a Master Password.");
		println!("This password will be used to encrypt your passwords.");
		println!("Please use a password that is very strong and you will remember.");
		println!(
			"If you lose or forget this password, these is no way to recover your passwords.\n"
		);

		let pass: String = ui::easypassword("Please enter your chosen Master Password: ");
		let hashed: String = sha512(pass);
		println!("The SHA512 of your Master Password is: {hashed}");
		add_to_meta(
			&Connection::open(getdbfile()).unwrap(),
			MetaEntry {
				key: String::from("master_password"),
				value: hashed,
			},
		);
		println!("\nMaster Password has been set. The program will now exit. Restart the program to use it.");
		std::process::exit(0);
	}

	Connection::open(getdbfile()).unwrap()
}

pub fn getdbfile() -> String {
	format!(
		"{}{}{}{}{}",
		dirs::home_dir().unwrap().display(),
		std::path::MAIN_SEPARATOR,
		".clipass",
		std::path::MAIN_SEPARATOR,
		"clipass.db"
	)
}

pub fn add_to_table(conn: &Connection, entry: Entry) -> bool {
	let exists: i64 = conn
		.query_row(
			"SELECT COUNT(*) FROM passwords WHERE service = ?1",
			[&entry.service],
			|row| row.get(0),
		)
		.unwrap();

	if exists > 0 {
		println!("Service already exists in database!");
		return false;
	}

	conn.execute(
		"INSERT INTO passwords (service, pass, note) VALUES (?1, ?2, ?3)",
		[&entry.service, &entry.pass, &entry.note],
	)
	.unwrap();

	true
}

pub fn get_from_table(conn: &Connection, service: String) -> Entry {
	let mut stmt = conn
		.prepare("SELECT * FROM passwords WHERE service = ?1")
		.unwrap();
	let entry_iter = stmt
		.query_map([service], |row| {
			Ok(Entry {
				service: row.get(0).unwrap(),
				pass: row.get(1).unwrap(),
				note: row.get(2).unwrap(),
			})
		})
		.unwrap();

	let mut entry: Entry = Entry {
		service: String::new(),
		pass: String::new(),
		note: String::new(),
	};

	for e in entry_iter {
		entry = e.unwrap();
	}

	entry
}

pub fn add_to_meta(conn: &Connection, entry: MetaEntry) -> bool {
	let exists: i64 = conn
		.query_row(
			"SELECT COUNT(*) FROM meta WHERE key = ?1",
			[&entry.key],
			|row| row.get(0),
		)
		.unwrap();

	if exists > 0 {
		println!("Key already exists in database!");
		return false;
	}

	conn.execute(
		"INSERT INTO meta (key, value) VALUES (?1, ?2)",
		[&entry.key, &entry.value],
	)
	.unwrap();

	true
}

pub fn get_from_meta(conn: &Connection, key: String) -> MetaEntry {
	let mut stmt = conn.prepare("SELECT * FROM meta WHERE key = ?1").unwrap();
	let entry_iter = stmt
		.query_map([key], |row| {
			Ok(MetaEntry {
				key: row.get(0).unwrap(),
				value: row.get(1).unwrap(),
			})
		})
		.unwrap();

	let mut entry: MetaEntry = MetaEntry {
		key: String::new(),
		value: String::new(),
	};

	for e in entry_iter {
		entry = e.unwrap();
	}

	entry
}

pub fn delete_from_table(conn: &Connection, service: String) -> bool {
	let exists: i64 = conn
		.query_row(
			"SELECT COUNT(*) FROM passwords WHERE service = ?1",
			[&service],
			|row| row.get(0),
		)
		.unwrap();

	if exists == 0 {
		println!("Service does not exist in database!");
		return false;
	}

	conn.execute("DELETE FROM passwords WHERE service = ?1", [&service])
		.unwrap();

	true
}

pub fn list_services(conn: &Connection) -> Vec<String> {
	let mut stmt = conn.prepare("SELECT service FROM passwords").unwrap();
	let entry_iter = stmt.query_map([], |row| Ok(row.get(0).unwrap())).unwrap();

	let mut entries: Vec<String> = Vec::new();

	for e in entry_iter {
		entries.push(e.unwrap());
	}

	entries
}

pub fn update_meta(conn: &Connection, entry: MetaEntry) -> bool {
	let exists: i64 = conn
		.query_row(
			"SELECT COUNT(*) FROM meta WHERE key = ?1",
			[&entry.key],
			|row| row.get(0),
		)
		.unwrap();

	if exists == 0 {
		println!("Key does not exist in database!");
		return false;
	}

	conn.execute(
		"UPDATE meta SET value = ?1 WHERE key = ?2",
		[&entry.value, &entry.key],
	)
	.unwrap();

	true
}

pub fn exists_in_table(conn: &Connection, service: String) -> bool {
	let exists: i64 = conn
		.query_row(
			"SELECT COUNT(*) FROM passwords WHERE service = ?1",
			[&service],
			|row| row.get(0),
		)
		.unwrap();

	if exists == 0 {
		return false;
	}

	true
}
