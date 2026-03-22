use super::utils::*;
use super::utils::Solution::*;

pub fn fractional_output(a: f64, b: f64, d: f64, one: bool) -> Solution
{
	if d.sqrt() != d.sqrt().floor() {
		irrational_fraction(a, b, d, one)
	} else {
		simple_fraction(a, b, d, one)
	}
}

fn simple_fraction(a: f64, b: f64, d: f64, one: bool) -> Solution
{
	let soln1: String;
	let mut soln2 = String::new();
	let mut numer: f64;
	let mut denom: f64;
	let result1 = (-b - d.sqrt()) / (2.0 * a);
	let result2 = (-b + d.sqrt()) / (2.0 * a);

	if result1 == result1.floor() && result2 == result2.floor() {
		soln1 = numerical_solution(a, b, d);
		if !one { soln2 = numerical_solution(a, b, -d); }
	} else if result1 == result1.floor() {
		soln1 = numerical_solution(a, b, d);
		(numer, denom) = simplify(-b + d.sqrt(), 2.0 * a);
		soln2 = format!(" {numer:.3} \n-----\n {denom:.3}");
	} else if !one && result2 == result2.floor() {
		(numer, denom) = simplify(-b - d.sqrt(), 2.0 * a);
		soln1 = format!(" {numer:.3} \n-----\n {denom:.3}");
		soln2 = numerical_solution(a, b, d);
	} else {
		(numer, denom) = simplify(-b - d.sqrt(), 2.0 * a);
		soln1 = format!(" {numer:.3} \n-----\n {denom:.3}");
		if !one {
			(numer, denom) = simplify(-b + d.sqrt(), 2.0 * a);
			soln2 = format!(" {numer:.3} \n-----\n {denom:.3}");
		}
	}
	if one { FOne(soln1)}
	else { FTwo(soln1, soln2)}
	// todo!("simple fraction");
}

fn irrational_fraction(a: f64, b: f64, d: f64, one: bool) -> Solution
{
	let numer1 = format!(" {:.3} + √{d} ", -b);
	let width = numer1.len();
	let line = "-".repeat(width).to_string();
	let denom = format!("{:^width$.3}", 2.0 * a);
	let soln1 = numer1 + "\n" + &line + "\n" + &denom;
	if one {
		return FOne(soln1);
	}
	let numer2 = format!(" {:.3} - √{d} ", -b);
	let soln2 = numer2 + "\n" + &line + "\n" + &denom;
	FTwo(soln1, soln2)
}

fn numerical_solution(a: f64, b: f64, d: f64) -> String
{
	format!("{}", (-b - d.sqrt()) / (2.0 * a))
}

fn simplify(numer: f64, denom: f64) -> (f64, f64)
{
	let factor = gcd(numer, denom);
	(numer / factor, denom / factor)
}