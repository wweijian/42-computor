use std::collections::{HashMap};

pub fn validate(eqn: &str) -> Option<HashMap<i32, Vec<f64>>>
{
	let mut v: HashMap<i32, Vec<f64>> = HashMap::new();
	let mut chars = eqn.chars().peekable();
	
	if cfg!(debug_assertions) {dbg!(eqn);}
	while let Some(c) = chars.peek() {
		match c {
			'0'..='9' | '.' => {
				dbg!(c);
			}
			'+' | '-' => {
				dbg!(c);
			}
			'X' => {
				dbg!(c);
			}
			_ => { return Some(v);}
		}
		chars.next();
	}
	return None;
}
