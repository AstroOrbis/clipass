use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use rand::Rng;

pub fn encrypt(password: String, data: String) -> String {
	let mc = new_magic_crypt!(password, 256);
	mc.encrypt_str_to_base64(data)
}

pub fn decrypt(password: String, encrypted_data: String) -> String {
	let mc = new_magic_crypt!(password, 256);
	mc.decrypt_base64_to_string(encrypted_data).unwrap()
}

pub fn genpass() -> String {
	let length = 16;
	let charset = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*()_+~`|}{[]:;?><,./-=";
	let mut rng = rand::thread_rng();
	let mut pass: String = String::new();
	for _ in 0..length {
		pass.push(
			charset
				.chars()
				.nth(rng.gen_range(0..charset.len()))
				.unwrap(),
		);
	}
	pass
}
