use poise::serenity_prelude::Colour;

pub enum Colors {
	Blue,
	Orange,
	Red,
}

impl From<Colors> for Colour {
	fn from(val: Colors) -> Self {
		match val {
			Colors::Blue => Colour::from((136, 199, 253)),
			Colors::Orange => Colour::from((255, 179, 74)),
			Colors::Red => Colour::from((255, 94, 74)),
		}
	}
}
