use crate::consts::*;
use bottomify::bottom::{decode_string, encode_string};
use include_dir::{include_dir, Dir};
use rand::seq::SliceRandom;
use std::collections::HashMap;
use std::vec;

const FILES: Dir = include_dir!("src/copypastas");

pub fn parse_snowflake_from_env<T, F: Fn(u64) -> T>(key: &str, f: F) -> Option<T> {
	std::env::var(key)
		.ok()
		.and_then(|v| u64::from_str_radix(&v, 10).map(&f).ok())
}
pub fn parse_snowflakes_from_env<T, F: Fn(u64) -> T>(key: &str, f: F) -> Option<Vec<T>> {
	std::env::var(key).ok().and_then(|gs| {
		gs.split(',')
			.map(|g| u64::from_str_radix(g, 10).map(&f))
			.collect::<Result<Vec<_>, _>>()
			.ok()
	})
}
/*
 * chooses a random element from an array
 */
async fn random_choice<const N: usize>(arr: [&str; N]) -> String {
	let mut rng = rand::thread_rng();
	let resp = arr.choose(&mut rng).expect("couldn't choose random value!");
	resp.to_string()
}

/*
 * pub functions to get random elements
 * from our consts
 */

pub async fn get_random_response() -> String {
	random_choice(RESPONSES).await
}

pub async fn get_random_lore() -> String {
	random_choice(LORE).await
}

// waiting for `round_char_boundary` to stabilize
pub fn floor_char_boundary(s: &str, index: usize) -> usize {
	if index >= s.len() {
		s.len()
	} else {
		let lower_bound = index.saturating_sub(3);
		let new_index = s.as_bytes()[lower_bound..=index]
			.iter()
			.rposition(|&b| (b as i8) >= -0x40); // b.is_utf8_char_boundary

		// Can be made unsafe but whatever
		lower_bound + new_index.unwrap()
	}
}
// waiting for `int_roundings` to stabilize
fn div_ceil(a: usize, b: usize) -> usize {
	(a + b - 1) / b
}

/*
 * splits a message into multiple parts so that
 * it can fit discord's character limit
 */
fn split_msg(mut msg: String) -> Vec<String> {
	const CHAR_LIMIT: usize = 2000;
	let mut msgs = Vec::with_capacity(div_ceil(msg.len(), CHAR_LIMIT));

	while msg.len() > CHAR_LIMIT {
		msgs.push(msg.split_off(floor_char_boundary(&msg, CHAR_LIMIT)));
	}
	msgs
}

/*
 * gets a random copypasta from include/
 */
pub async fn get_copypasta(name: &str) -> Vec<String> {
	let mut files: HashMap<&str, &str> = HashMap::new();

	for file in FILES.files() {
		let name = file.path().file_stem().unwrap().to_str().unwrap();

		let contents = file.contents_utf8().unwrap();

		// refer to files by their name w/o extension
		files.insert(name, contents);
	}

	if files.contains_key(&name) {
		let reply = files[name].to_string();
		split_msg(reply)
	} else {
		vec![format!("couldn't find {name:?} in files")]
	}
}

/*
 * encodes a message into bottom
 */
pub async fn bottom_encode(msg: &str) -> String {
	encode_string(&msg)
}

/*
 * decodes a bottom string into english
 */
pub async fn bottom_decode(msg: &str) -> String {
	let decoded = decode_string(&msg);
	match decoded {
		Ok(ret) => ret,
		Err(why) => {
			println!("couldn't decode {:?}! ({:?})", msg, why);
			"couldn't decode that! sowwy 🥺".to_string()
		}
	}
}

/*
 * converts celsius to fahrenheit
 */
pub fn celsius_to_fahrenheit(c: f64) -> f64 {
	(c * (9.0 / 5.0)) + 32.0
}

/*
 * converts fahrenheit to celsius
 */
pub fn fahrenheit_to_celsius(f: f64) -> f64 {
	(f - 32.0) * (5.0 / 9.0)
}
