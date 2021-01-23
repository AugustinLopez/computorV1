#[derive(PartialEq)]
pub enum Expected {
	Nothing,
	Sign,
	Multiplier,
	Star,
	X,
	Hat,
	Power,
	Error,
}

pub struct Item {
	pub sign: f64,
	pub multiplier: f64,
	pub exponent: i64,
}

pub fn nothing(equation: &str, item: &mut Item) -> Expected {
	match equation {
		"+" | "*" | "^" => return Expected::Error,
		"-" => {
			item.sign = -1.0;
			return Expected::Multiplier;
		},
		"X" | "x" => {
			item.multiplier = 1.0;
			item.exponent = 1;
			return Expected::Hat;
		},
		_ => {
			let try_parsing = equation.parse::<f64>();
			if try_parsing.is_err() {
				return Expected::Error;
			}
			item.multiplier = try_parsing.unwrap();
			return Expected::Star;
		}
	}
}

pub fn multiplier(equation: &str, item: &mut Item) -> Expected {
	match equation {
		"-" | "+" | "^" | "*" => {
			return Expected::Error;
		},
		"X" | "x" => {
			item.multiplier = 1.0;
			item.exponent = 1;
			return Expected::Hat;
		},
		_ => {
			let try_parsing = equation.parse::<f64>();
			if try_parsing.is_err() {
				return Expected::Error;
			}
			item.multiplier = try_parsing.unwrap();
			return Expected::Star;
		}
	}
}

pub fn star(equation: &str, item: &mut Item) -> Expected {
	match equation {
		"*" => return Expected::X,
		"X" | "x" => {
			item.exponent = 1;
			return Expected::Hat;
		}
		"+" => {
			item.exponent = 0;
			item.sign = 1.0;
			return Expected::Multiplier;
		}
		"-" => {
			item.exponent = 0;
			item.sign = -1.0;
			return Expected::Multiplier;
		}
		_ => return Expected::Error,
	}
}

pub fn function_x(equation: &str, item: &mut Item) -> Expected {
	match equation {
		"X" | "x" => {
			item.exponent = 1;
			return Expected::Hat;
		}
		_ => return Expected::Error,
	}
}

pub fn hat(equation: &str, item: &mut Item) -> Expected {
	match equation {
		"^" => return Expected::Power,
		"+" => {
			item.exponent = 1;
			item.sign = 1.0;
			return Expected::Multiplier;
		}
		"-" => {
			item.exponent = 1;
			item.sign = -1.0;
			return Expected::Multiplier;
		}
		_ =>return Expected::Error,
	}
}

pub fn sign(equation: &str, item: &mut Item) -> Expected {
	match equation {
		"-" => {
			item.sign = -1.0;
			return Expected::Multiplier;
		},
		"+" => {
			item.sign = 1.0;
			return Expected::Multiplier;
		},
		_ => return Expected::Error,
	}
}

pub fn power(equation: &str, item: &mut Item) -> Expected {
	let try_parsing = equation.parse::<i64>();
	if try_parsing.is_err() {
		return Expected::Error;
	}
	item.exponent = try_parsing.unwrap();
	return Expected::Sign;
}
