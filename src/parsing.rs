pub mod expected;
use expected::Expected as Expected;
use std::collections::BTreeMap;

fn keep_delimiters<'a>(text: &'a str, delim: &str) -> Vec<&'a str> {
	let mut result = Vec::new();
	let mut last = 0;
	for (index, matche) in text.match_indices(|c: char| delim.find(c) != None) {
		if last != index {
			result.push(&text[last..index]);
		}
	    result.push(matche);
	    last = index + matche.len();
	}
	if last < text.len() {
	    result.push(&text[last..]);
	}
	result
}

pub fn precheck_equation(equation: &String) -> i8 {
	if !equation.is_ascii() {
		eprintln!("Error: non-ascii characters");
		return -1;
	}
	let split_eq = keep_delimiters(&equation, "=");
	if equation.matches("=").count() != 1 {
		eprintln!("Error: equation must contains exactly one '=' character");
		return -1;
	} else if  split_eq.len() != 3 || split_eq[1] != "=" {
		eprintln!("Error: invalid use of '=' character");
		return -1;
	}
	return 0;
}

fn load_sign(sign: f64, leftpart: bool)-> f64 {
	if leftpart == true {
		sign
	} else {
		-sign
	}
}

fn load_result(iter: bool, sign: f64, element: &mut expected::Item, result: &mut BTreeMap<i64, f64>, debug: bool) {
	if iter == true {
		if debug == true {
			if result.contains_key(&element.exponent) {
				let v = result.get(&element.exponent);
				if let Some(x) = v {
					print!("\t{{ {} * X ^ {} }} =",  x, element.exponent);
				}
			} else {
				print!("\t{{}} =");
			}
		}
		*result.entry(element.exponent).or_insert(0.0) += element.multiplier * sign;
		if debug == true {
			let v = result.get(&element.exponent);
			if let Some(x) = v{
				println!("> {} * X ^ {}\n",  x, element.exponent);
			}
		}
		element.multiplier = 0.0;
		element.exponent = 0;
	}
}

pub fn load_data(equation: &String, leftpart: bool, mut result: &mut BTreeMap<i64, f64>, debug: bool) -> i8{
	let mut split = keep_delimiters(&equation, "=");
	let mut len_x = 0;
	let mut len_y = 0;
	if leftpart == true {
		split = keep_delimiters(&split[0], "+-*^xX");
	} else {
		split = keep_delimiters(&split[2], "+-*^xX");
		len_x = equation.chars().position(|c| c == '=').unwrap()+1;
		len_y = len_x;
	}
	let mut split = split.into_iter();
	let mut element = expected::Item {sign:1.0, multiplier:0.0, exponent: 0};
	let mut iter = Expected::Nothing;
	while let Some(s) = split.next() {
		len_x = len_y;
		len_y += s.len();
		let s = s.trim();
		if debug == true {
			println!("> {}[ {} ]{}", &equation[..len_x], s, &equation[len_y..]);
		}
		if s.len() == 0 {
			continue ;
		}
		match iter {
			Expected::Nothing => iter = expected::nothing(&s, &mut element),
			Expected::Multiplier => iter = expected::multiplier(&s, &mut element),
			Expected::Star => {
				let sign = load_sign(element.sign, leftpart);
				iter = expected::star(&s, &mut element);
				load_result(iter == Expected::Multiplier, sign, &mut element, &mut result, debug);
			}
			Expected::X => iter = expected::function_x(&s, &mut element),
			Expected::Hat => {
				let sign = load_sign(element.sign, leftpart);
				iter = expected::hat(&s, &mut element);
				load_result(iter == Expected::Multiplier, sign, &mut element, &mut result, debug);
			}
			Expected::Power => iter = expected::power(&s, &mut element),
			Expected::Sign => {
				let sign = load_sign(element.sign, leftpart);
				load_result(true, sign, &mut element, &mut result, debug);
				iter = expected::sign(&s, &mut element);
			},
			_ => (),
		}
		if iter == Expected::Error {
			eprintln!("Error: \"{}\": invalid item", s);
			eprintln!("> {}[ {} ]{}", &equation[..len_x], s, &equation[len_y..]);
			return -1;
		}
	}
	match iter {
		Expected::Star | Expected::Hat | Expected::Sign => {
				let sign = load_sign(element.sign, leftpart);
				load_result(true, sign, &mut element, &mut result, debug);
		},
		_ => {
			eprintln!("Error: \"{}\": invalid item", &equation[len_x..len_y]);
			eprintln!("> {}[ {} ]{}", &equation[..len_x], &equation[len_x..len_y], &equation[len_y..]);
			return -1;
		}
	}
	0
}

pub fn print_data(result: &BTreeMap<i64, f64>){
	let mut count: u64 = 0;
	let mut alert = false;
	print!("Reduced form: ");
	for (exponent, multiplier) in result.iter() {
		let mut multiplier = multiplier.clone();
		let exponent = exponent.clone();
		if multiplier != 0.0 {
			if multiplier < 0.0 {
				if count == 0 {
					print!("-");
				} else {
					print!("- ");
				}
				multiplier = -multiplier;
			}
			else if count != 0 {
				print!("+ ");
			}
			if multiplier.abs() >= 9007199254740991.0 {
				alert = true;
			}
			if exponent != 0 || multiplier != 0.0 {
				count += 1;
			}
			if exponent != 0 && multiplier != 1.0 {
				print!("{} * ", multiplier);
			} else if exponent == 0 && multiplier != 0.0 {
				print!("{} ", multiplier);
			}
			if exponent == 1 {
				print!("X ");
			} else if exponent != 0 {
				print!("X ^ {} ", exponent);
			}
		}
	}
	if count == 0 {
		println!("0 = 0");
	} else {
		println!("= 0");
	}
	if alert == true {
		println!("<!> Coefficients(s) above the safe integer limit (2^53-1)");
	}
}
