mod parse;

fn main() {
	let args: Vec<String> = std::env::args().collect();

	if args.len() != 2 {
		eprintln!("Usage: {} <equation>", args[0]);
		std::process::exit(1);
	}
	if parse::validate(&args[1]).is_some() {
		println!("success!");
	} else {
		println!("failure");
	}
}