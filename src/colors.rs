use poise::serenity_prelude::Colour;

pub enum Colors {
	Blue,
}

impl From<Colors> for Colour {
	fn from(val: Colors) -> Self {
		match val {
			Colors::Blue => Colour::from((136, 199, 253)),
		}
	}
}
