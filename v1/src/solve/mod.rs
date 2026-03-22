#![allow(unused_variables, dead_code)]

mod print;
mod fraction;
mod utils;

use std::process::ExitCode;
use std::collections::HashMap;
use print::*;
use utils::*;
use utils::Solution::*;

#[cfg(feature = "fraction")]
use fraction::fractional_output;

pub fn solve (map: &mut HashMap<i32, f64>) -> Result<ExitCode, String>
{
	let (a, b, c) = get_eqn_values(map);
	print_degree(a,b,c);
	print_poly(a, b, c);
	let discriminant = b * b - 4.0 * a * c;
	if a == 0.0 && b == 0.0 && c == 0.0 {
		print_solution(Inff);
	} else if discriminant < 0.0 && a != 0.0 {
		print_solution(NoReal);
	} else if a == 0.0 && b == 0.0 && c != 0.0 {
		print_solution(NoSoln);
	} else if a == 0.0 && b != 0.0 {
		print_solution(solve_linear(b, c));
	} else {
		print_solution(general_formula(a, b, c, discriminant));
		print_discriminant(discriminant);
	}
	return Ok(ExitCode::SUCCESS);
}

fn solve_linear(b: f64, c: f64) -> Solution
{
	return One(c / b * -1.0);
}

fn general_formula(a: f64, b: f64, c: f64, d: f64) -> Solution
{
	let pos = d.sqrt();
	let neg = -pos;

	if a == 0.0 { panic!("a is 0") };
	let x1 = (-b + pos) / (2.0 * a);
	let x2 = (-b + neg) / (2.0 * a);
	#[cfg(feature = "fraction")]
		return fractional_output(a, b, d, x1 == x2);
	#[cfg(not(feature = "fraction"))]
	{
		if x1 == x2 {
			return One(x1);
		} else {
			return Two(x1, x2);
		}
	}
}
