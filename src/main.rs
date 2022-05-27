use std::env;


fn main() {
	let args: Vec<String> = env::args().collect();
	
	if args.len() != 1+5 {
		print_info();
	} else {
		let input   = args[1].parse::<f64>().expect("Input 1 is not a number.");
		let in_min  = args[2].parse::<f64>().expect("Input 2 is not a number.");
		let in_max  = args[3].parse::<f64>().expect("Input 3 is not a number.");
		let out_min = args[4].parse::<f64>().expect("Input 4 is not a number.");
		let out_max = args[5].parse::<f64>().expect("Input 5 is not a number.");
		
		do_the_thing_xdlmao(input, in_min, in_max, out_min, out_max);
	}
}

fn print_info() {
	println!("Pass arguments to calculate. Order:");
	println!("input, in min, in max, out min, out max.");
	println!("Example:");
	println!("remap 0.75 0 1 -100 100");
	println!("> 50");
}

fn do_the_thing_xdlmao(input: f64, in_min: f64, in_max: f64, out_min: f64, out_max: f64) {
	println!("Input: {input}");
	println!("In:   {in_min} ... {in_max}");
	println!("Out:  {out_min} ... {out_max}");
	
	println!("> {}", remap(&input, &in_min, &in_max, &out_min, &out_max));
}

fn remap(input: &f64, in_min: &f64, in_max: &f64, out_min: &f64, out_max: &f64) -> f64{
	out_min + (input-in_min) / (in_max-in_min) * (out_max-out_min)
}