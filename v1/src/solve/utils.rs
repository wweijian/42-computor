use crate::solve::HashMap;

pub enum Solution {
	NoReal,
	NoSoln,
	One(f64),
	Two(f64, f64),
	Inff,
	FOne(String),
	FTwo(String, String),
}

pub fn get_eqn_values(map: &HashMap<i32, f64>) -> (f64, f64, f64)
{
	return (map.get(&2).copied().unwrap_or(0.0),
			map.get(&1).copied().unwrap_or(0.0),
			map.get(&0).copied().unwrap_or(0.0));
}

pub fn gcd(a: f64, b: f64) -> f64 {
	if b == 0.0 { a } else { gcd(b, a % b) }
}
