mod parse;

use std::process::ExitCode;

fn main() -> Result<ExitCode, String>
{
	let args: Vec<String> = std::env::args().collect();

	if args.len() != 2 {
		eprintln!("Usage: {} <equation>", args[0]);
		// should take from stdin 
		std::process::exit(1);
	}
	let map = parse::validate(&args[1])?;
	println!("{:#?}", map);
	if !map.is_empty() {
		println!("success!");
		Ok(ExitCode::SUCCESS)
	} else {
		println!("failure");
		Ok(ExitCode::FAILURE)
	}
}