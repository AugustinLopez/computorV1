pub enum RetVal {
	GoToNext,
	StopIter,
	StopAll,
	NextIsStopIter,
	IsError,
}

type ODebug = bool;
type OLcm = bool;

pub fn argument_option(argument: &String, option: &mut(ODebug, OLcm)) -> RetVal {
	if argument.chars().nth(0) != Some('-') {
		return RetVal::StopIter;
	} else if argument.chars().nth(1) == None {
		eprintln!("Error: '' is an invalid argument");
		return RetVal::IsError;
	} else if argument.chars().nth(1) == Some('-')
	&& argument.chars().nth(2) == None {
		return RetVal::NextIsStopIter;
	}
	match argument.chars().nth(1) {
		Some(c) => {
			if c.is_ascii_digit() || c == ' ' || c == 'X' || c == 'x' {
				return RetVal::StopIter;
			}
		}
		_ => return RetVal::IsError,
	}
	for chars in argument.chars().skip(1) {
		match chars {
			'd' => option.0 = true,
			'l' => option.1 = true,
			'h' => return RetVal::StopAll,
			_ => {
				eprintln!("Error: '{}' is an invalid argument", chars);
				return RetVal::IsError;
			},
		}
	}
	return RetVal::GoToNext;
}

/*
usize: argc
bool1: option d
bool2: option l
bool3: error
*/

pub fn argument_parsing(args: &Vec<String>) -> (usize, bool, bool, bool) {
	let mut ret = (0, false, false, false);
	let mut option = (false, false);
	for (i, item) in args.iter().skip(1).enumerate() {
		match argument_option(&item, &mut option) {
			RetVal::GoToNext => (),
			RetVal::IsError => {
				ret.3 = true;
				break;
			},
			RetVal::StopIter => {
				ret.0 = i + 1;
				break;
			},
			RetVal::NextIsStopIter => {
				ret.0 = i + 2;
				break;
			},
			RetVal::StopAll => {
				ret.0 = 0;
				break;
			},
		}
	}
	ret.1 = option.0;
	ret.2 = option.1;
	ret
}
