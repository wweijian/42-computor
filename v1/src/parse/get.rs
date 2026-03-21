use crate::parse::Chars;
use crate::parse::{ ERR_EQN, ERR_MAL, ERR_HANG, ERR_DIV_ZERO };
use crate::parse::utils::*;

use std::iter::Peekable;

pub fn get_coeff(chars: &mut Peekable<Chars<'_>>, div: bool) -> Result<f64, String>
{
	let mut	s = String::new();
	let mut dec = false;
	let coeff: f64;
	
	while let Some(c) = chars.peek() {
		if dec && *c == '.' { dbg!(c); return Err(format!("{ERR_EQN}: '{c}'")); }
		else if *c == '.' { dec = true; }
		else if *c == ' ' { chars.next(); continue; }
		else if matches!(*c, 'X' | 'x' ) || is_operation(Some(c)) { break ; }
		else if !c.is_ascii_digit() { dbg!(c); return Err(format!("{ERR_EQN}: '{c}'")) }
		s.push(*c);
		chars.next();
	}
	coeff = s.parse::<f64>().map_err(|e| e.to_string())?;
	if div {
		dbg!("div by zero");
		if coeff == 0.0 { dbg!(); return Err(ERR_DIV_ZERO.to_string()); }
		return Ok(1.0 / coeff);
	} else {
		return Ok(coeff);
	}
}

pub fn get_deg(chars: &mut Peekable<Chars<'_>>, div: bool, deg: i32) -> Result<i32, String>
{
	chars.next();
	while let Some(c) = chars.peek() {
		match c {
			' ' => { chars.next(); continue; }
			'+' | '-' | '*' | '/' | '=' => { return Ok(deg + 1) }
			'^' => { return get_deg_val(chars, div, deg); }
			_ => { dbg!(c); return Err(format!("{ERR_EQN}: '{c}'")) }
		}
	}
	return Ok(deg + 1) ;
}

pub fn get_deg_val(chars: &mut Peekable<Chars<'_>>, div: bool, mut deg: i32) -> Result<i32, String>
{
	let mut s = String::new();

	chars.next();
	if chars.peek().is_none() || is_operation(chars.peek()) { return Err(ERR_HANG.to_string()); }
	while let Some(c) = chars.peek() {
		match c {
			'0'..='9' => { s.push(*c); }
			' ' => {}
			'+' | '-' | '*' | '/' | '=' => { deg += s.parse::<i32>().map_err(|e| e.to_string())?; return Ok(deg); }
			_ => { dbg!(c); return Err(format!("{ERR_EQN}: '{c}'")) }
		}
		chars.next();
	}
	deg += s.parse::<i32>().map_err(|e| e.to_string())?;
	match div {
		true => { deg *= -1 },
		_ => {},
	};
	return Ok(deg);
}

pub fn get_polarity (chars: &mut Peekable<Chars<'_>>) -> Result<f64, String>
{
	let polarity = match chars.peek() {
		Some('-') => -1.0,
		_ => 1.0,
	};
	chars.next();
	skip_space(chars);
	let c = chars.peek();
	if is_end(c) {
		dbg!(); 
		return Err(ERR_HANG.to_string());
	}
	if is_operation(c) {
		dbg!(*c.unwrap()); 
		return Err(format!("{ERR_MAL} near '{}'", *c.unwrap())); 
	}
	return Ok(polarity);
}

pub fn get_reciprocal(chars: &mut Peekable<Chars<'_>>) -> Result<bool, String>
{
	let reciprocal = match chars.peek() {
		Some('*') => false,
		_ => true,
	};
	chars.next();
	skip_space(chars);
	let c = chars.peek();
	if is_end(c) {
		dbg!(); 
		return Err(ERR_HANG.to_string());
	}
	if is_operation(c) {
		dbg!(*c.unwrap()); 
		return Err(format!("{ERR_MAL} near '{}'", *c.unwrap())); 
	}
	return Ok(reciprocal);
}