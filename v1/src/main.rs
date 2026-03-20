mod parse;
mod solve;

use std::process::ExitCode;
use solve::solve;

fn main() -> Result<ExitCode, String>
{
	let args: Vec<String> = std::env::args().collect();

	if args.len() != 2 {
		eprintln!("Usage: {} <equation>", args[0]);
		// should take from stdin 
		std::process::exit(1);
	}
	let mut map = parse::populate_map(&args[1])?;
	println!("{:#?}", map);
	return solve(&mut map);
}