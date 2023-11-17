use crate::{consts, Error};

use once_cell::sync::Lazy;
use poise::serenity_prelude::GuildId;
use rand::seq::SliceRandom;

pub fn parse_snowflake_from_env<T, F: Fn(u64) -> T>(key: &str, f: F) -> Option<T> {
	std::env::var(key).ok().and_then(|v| v.parse().map(&f).ok())
}
pub fn parse_snowflakes_from_env<T, F: Fn(u64) -> T>(key: &str, f: F) -> Option<Vec<T>> {
	std::env::var(key).ok().and_then(|gs| {
		gs.split(',')
			.map(|g| g.parse().map(&f))
			.collect::<Result<Vec<_>, _>>()
			.ok()
	})
}
/*
 * chooses a random element from an array
 */
pub fn random_choice<const N: usize>(arr: [&str; N]) -> Result<String, Error> {
	let mut rng = rand::thread_rng();
	if let Some(resp) = arr.choose(&mut rng) {
		Ok((*resp).to_string())
	} else {
		Err(Into::into("couldn't choose from arr!"))
	}
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

pub fn is_guild_allowed(gid: GuildId) -> bool {
	static ALLOWED_GUILDS: Lazy<Vec<GuildId>> = Lazy::new(|| {
		parse_snowflakes_from_env("ALLOWED_GUILDS", GuildId)
			.unwrap_or_else(|| vec![consts::TEAWIE_GUILD, GuildId(1091969030694375444)])
	});

	ALLOWED_GUILDS.contains(&gid)
}
