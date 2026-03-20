mod utils;
mod get;

use std::collections::HashMap;
use std::iter::Peekable;
use std::str::Chars;

use utils::*;
use get::*;

pub const ERR_EQN: &str = "unexpected character";
pub const ERR_MAL: &str = "malformed equation";
pub const ERR_HANG: &str = "hanging operator";
pub const ERR_DIV_ZERO: &str = "division by zero";
pub const ERR_BAD_DEG: &str = "incorrect degrees of equation";

pub fn populate_map(eqn: &str) -> Result<HashMap<i32, f64>, String>
{
	let mut map: HashMap<i32, f64> = HashMap::new();
	let mut chars = eqn.chars().peekable();

	if cfg!(debug_assertions) {dbg!(eqn);}
	while let Some(c) = chars.peek() {
		match c {
			'0'..='9' | '+' | '-' | 'X' | 'x' => {
				let term = get_indeterminate(&mut chars)?;
				*map.entry(term.0).or_insert(0.0) += term.1
			}
			' ' => { chars.next(); }
			'=' => { break ; }
			_ => { dbg!(); return Err(format!("{ERR_EQN}: '{c}'")); }
		}
	}
	if chars.peek().is_none() {
		return is_valid_equation(map);
	}
	let remainder: String = chars.clone().collect();
	dbg!(&remainder);
	chars.next();
	while let Some(c) = chars.peek() {
		match c {
			'0'..='9' | '+' | '-' | 'X' | 'x' => {
				let term = get_indeterminate(&mut chars)?;
				*map.entry(term.0).or_insert(0.0) -= term.1
			}
			' ' => { chars.next(); }
			_ => { dbg!(); return Err(format!("{ERR_EQN}: '{c}'")); }
		}
	}
	return is_valid_equation(map);
}

fn get_indeterminate(chars: &mut Peekable<Chars<'_>>) -> Result<(i32, f64), String>
{
	let mut deg: i32 = 0;
	let mut coeff: f64 = 1.0;
	let mut div = false;

	while let Some(c) = chars.peek() {
		match c {
			'0'..='9' => {
				coeff *= get_coeff(chars, div)?;
				if is_end_term(chars.peek()) { return Ok((deg, coeff)); }
			}
			'+' | '-' => {
				coeff *= get_polarity(chars)?;
			}
			'X' | 'x' => {
				deg = get_deg(chars, div, deg)?;
				if is_end_term(chars.peek()) { return Ok((deg, coeff)); }
			}
			'*' | '/' => {
				div = get_reciprocal(chars)?;
			}
			' ' => {chars.next();}
			'=' => {return Ok((deg, coeff));}
			_ => {dbg!(); return Err(format!("{ERR_EQN}: '{c}'"))}
		}
	}
	return Ok((deg, coeff));
}

fn is_valid_equation(map: HashMap<i32, f64>) -> Result<HashMap<i32, f64>, String>
{
	for key in map.keys() {
		if *key > 2 || *key < 0 {
			dbg!(*key);
			return Err(ERR_BAD_DEG.to_string());
		}
	}
	return Ok(map);
}