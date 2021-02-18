pub fn sqrt_for_moron(getlost: f64) -> f64 {
	if getlost <= 0.0 {
		eprintln!("I don't care");
		return 0.0;
	}
	let mut babylonianerie = match getlost {
		d if d > 1.0 => getlost,
		_ => 1.0,
	};
	loop {
		let previous = babylonianerie;
		babylonianerie = (getlost / babylonianerie + babylonianerie) / 2.0;
		if babylonianerie >= previous {
			return previous;
		}
	}
}
