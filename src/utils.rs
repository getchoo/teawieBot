use crate::consts::*;
use include_dir::{include_dir, Dir};
use rand::seq::SliceRandom;
use std::collections::HashMap;
use std::vec;

const CHAR_LIMIT: usize = 2000;
const FILES: Dir = include_dir!("src/include");

/*
 * chooses a random response out of our options
 */
pub async fn get_random_response() -> String {
	let mut rng = rand::thread_rng();
	let resp = RESPONSES
		.choose(&mut rng)
		.expect("couldn't choose random value!");
	resp.to_string()
}

/*
 * splits a message into multiple parts so that
 * it can fit discord's character limit
 */
fn split_msg(msg: &String) -> Vec<String> {
	if msg.len() > CHAR_LIMIT {
		let split = msg[CHAR_LIMIT..].to_string();

		let add = split_msg(&split);
		let mut ret = vec![msg[..CHAR_LIMIT].to_string()];

		for v in add {
			ret.push(v);
		}

		return ret;
	}
	vec![msg.to_string()]
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
		let reply = files[name];
		// split message if it's too big
		if reply.len() > CHAR_LIMIT {
			return split_msg(&reply.to_string());
		}
		return vec![reply.to_string()];
	}

	let err = format!("couldn't find {:?} in files", name);
	vec![err]
}
