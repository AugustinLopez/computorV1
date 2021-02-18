pub mod parsing;
pub mod option;
pub mod insult_to_good_practice;
use insult_to_good_practice::sqrt_for_moron as sqrt;
use std::collections::BTreeMap;

pub fn degree(result: &BTreeMap<i64, f64>) -> i64 {
	let mut degree = 0;
	for (exponent, multiplier) in result.iter() {
		let exponent = exponent.clone();
		let multiplier = multiplier.clone();
		if multiplier == 0.0 {
			continue ;
		}
		degree = exponent;
	}
	degree
}

pub fn degree_other(result: BTreeMap<i64, f64>, degree: i64){
	if degree < 0 {
		eprintln!("Error: a polynomial degree cannot be lower than 0");
	} else if degree > 2 {
		println!("The polynomial degree is strictly greater than 2. I Can't solve.");
	} else if degree == 0 {
		match result.get(&0) {
			Some(x) => println!("The equation is {}", x == &0.0),
			None => println!("The equation is true"),
		}
	}
}

pub fn gcd(a: i64, b: i64) -> i64 {
	let mut a = if a < 0 {
		a as i64
	} else {
		-a as i64
	};
	let mut b = if b < 0 {
		b as i64
	} else {
		-b as i64
	};
	while b != 0 {
		let t = b;
		b = a % b;
		a = t;
	}
	a
}

pub fn lcm(mut a: f64, mut b: f64) -> i64 {
	if a == 0.0 || b == 0.0 {
		return -1;
	}
	let solution = -a/b;
	if solution == (solution as i64) as f64 {
		return -1;
	}
	let i = 2.0;
	let mut ak = a * i;
	let mut bk = b * i;
	while a.fract() != 0.0 || b.fract() != 0.0 {
		a = ak;
		b = bk;
		ak = ak * i;
		bk = bk * i;
	}
	if a == (a as i64) as f64 && b == (b as i64) as f64 {
		let a = a as i64;
		let b = b as i64;
		if a.checked_mul(b) == None {
			return -1;
		}
		let lcm = (a * b) / gcd(a, b);
		let mut den = lcm / a;
		let mut nom = -lcm / b;
		if den < 0 {
			den = -den;
			nom = -nom;
		}
		println!("{}/{}", nom, den);
		return 0;
	} else {
		return -1;
	}
}

pub fn solution(a: f64, b: f64, try_lcm: bool) {
	if try_lcm == true && lcm(a, b) == 0 {
		return ;
	}
	let solution = -a/b;
	let as_integer = solution as i64;
	if solution.fract() == 0.0 && as_integer as f64 == solution {
		println!("{}", as_integer);
	} else {
		println!("{}", solution);
	}
}

pub fn degree_1(result: BTreeMap<i64, f64>, lcm: bool){
	let a: f64 = match result.get(&0) {
		Some(p) => *p,
		None => 0.0,
	};
	let b: f64 = match result.get(&1) {
		Some(p) => *p,
		None => 1.0,
	};
	println!("The solution is:");
	solution(a, b, lcm);
}

pub fn degree_2(result: BTreeMap<i64, f64>, lcm: bool){
	let a: f64 = match result.get(&0) {
		Some(p) => *p,
		None => 0.0,
	};
	let b: f64 = match result.get(&1) {
		Some(p) => *p,
		None => 0.0,
	};
	let c: f64 = match result.get(&2) {
		Some(p) => *p,
		None => 0.0,
	};
	let deter:f64 = (b*b)-4.0*a*c;
	if deter == 0.0 {
		println!("Discriminant is zero. The solution is:");
		solution(b, 2.0*c, lcm);
	} else if deter > 0.0 {
		println!("Discriminant is strictly positive. The 2 solutions are:");
		solution(b+sqrt(deter), 2.0*c, lcm);
		solution(b-sqrt(deter), 2.0*c, lcm);
	} else {
		println!("Discriminant is strictly negative. The 2 solutions are:");
		let imag = sqrt(-deter)/(2.0*c);
		let real = -b/(2.0*c);
		let imag = if imag > 0.0 {
			imag
		} else {
			-imag
		};
		if real != 0.0 {
			println!("{} + {} * i", real, imag);
			println!("{} - {} * i", real, imag);
		} else {
			println!("{} * i", imag);
			println!("-{} * i", imag);
		}
	}
}

pub fn compute(equation: &String, debug: bool, lcm: bool) -> i8 {
	if debug == true {
		println!("==> Equation: \"{}\"", equation);
	}
	if parsing::precheck_equation(equation) == -1 {
		return -1;
	}
	let mut result = BTreeMap::new();
	if parsing::load_data(equation, true, &mut result, debug) == -1
	|| parsing::load_data(equation, false, &mut result, debug) == -1 {
		return -1;
	}
	parsing::print_data(&result);
	let degree = degree(&result);
	println!("Polynomial degree: {}", degree);
	match degree {
		1 => degree_1(result, lcm),
		2 => degree_2(result, lcm),
		_ => degree_other(result, degree),
	}
	0
}

pub fn usage(args: &Vec<String>) {
	match args[0].rfind('/') {
		Some(num) => println!("Usage: ./{} [-dlh] \"argument\"", &args[0][num+1..]),
		_ => println!("Usage: ./{} [-dlh] \"argument\"", &args[0]),
	}
	println!("\t-d (debug)\t print step-by-step analysis \
			\n\t-l (lcm)  \t try printing irreductible fractions \
			\n\t-h (help) \t print usage");
}

pub fn computor_v1() -> i8 {
	let args: Vec<String> = std::env::args().collect();

	if args.len() < 2 {
		usage(&args);
		return 0;
	}
	let argc = option::argument_parsing(&args);
	if argc.3 == true {
		return -1;
	} else if argc.0 == 0 {
		usage(&args);
		return 0;
	}
	return compute(&args[argc.0], argc.1, argc.2);
}

fn main() {
	computor_v1();
}
