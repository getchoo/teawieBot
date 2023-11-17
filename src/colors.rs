use poise::serenity_prelude::Colour;

pub enum Colors {
	Blue,
}

impl Into<Colour> for Colors {
	fn into(self) -> Colour {
		match self {
			Colors::Blue => Colour::from((136, 199, 253)),
		}
	}
}
