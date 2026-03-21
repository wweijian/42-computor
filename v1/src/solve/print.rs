use crate::solve::Solution;
use crate::solve::Solution::*;

pub fn print_degree(a: f64, b:f64, c:f64) -> ()
{
	if a != 0.0 {
		println!("Degree: 2");
	} else if b != 0.0 {
		println!("Degree: 1");
	} else {
		println!("Degree: 0");
	}
}

pub fn print_poly(a: f64, b: f64, c: f64) -> ()
{
	let v = vec!(a, b, c);
	let mut parts: Vec<String> = vec![];
	for (degree, coeff) in v.into_iter().enumerate() {
		if coeff == 0.0 {
			continue ;
		} else if parts.is_empty() {
			parts.push(format!("{}{}", resolve_coeff(coeff), resolve_x(degree)));
		} else if coeff < 0.0 {
			parts.push(format!("- {}{}", resolve_coeff((coeff).abs()), resolve_x(degree)));
		} else {
			parts.push(format!("+ {}{}", resolve_coeff(coeff), resolve_x(degree)));
		}
	}
	println!("{}", parts.join(" "));
}

fn resolve_x (degree: usize) -> String
{
	if degree == 0 {
		return "".to_string();
	}
	return format!("x^{}", degree);
}

fn resolve_coeff (coeff: f64) -> String
{
	if coeff == 1.0 {
		return "".to_string();
	} else {
		return format!("{}", coeff);
	}
}

pub fn print_solution(soln: Solution) -> ()
{
	match soln {
		NoReal => {
			println!("X has no real solutions");
		}
		NoSoln => {
			println!("X has no solutions");
		}
		One(x) | FOne(x)=> {
			println!("X has one solution: {x}");
		}
		Two(x1, x2) | FTwo(x1, x2) => {
			println!("X has two solution: {x1} and {x2}");
		}
		Inff => {
			println!("X has infinite number of solutions");
		}
	}
}

pub fn print_discriminant(d: f64) -> ()
{
	println!("discriminant: {d}");
}