use std::{env, cmp::max};
use pad::PadStr;

fn main() {
	let args: Vec<String> = env::args().collect();
	
	if args.len() != 1+5 {
		show_info();
	} else {
		let input   = args[1].parse::<f64>().expect("\n\n/!\\ Argument 1 is not a number!\n\n");
		let in_min  = args[2].parse::<f64>().expect("\n\n/!\\ Argument 2 is not a number!\n\n");
		let in_max  = args[3].parse::<f64>().expect("\n\n/!\\ Argument 3 is not a number!\n\n");
		let out_min = args[4].parse::<f64>().expect("\n\n/!\\ Argument 4 is not a number!\n\n");
		let out_max = args[5].parse::<f64>().expect("\n\n/!\\ Argument 5 is not a number!\n\n");
		
		show_result(input, in_min, in_max, out_min, out_max);
	}
}

fn show_info() {
	println!("Pass arguments to calculate. Order:");
	println!("input, in min, in max, out min, out max.");
	println!("Example:");
	println!("remap 0.75 0 1 -100 100");
	println!("> 50");
}

fn show_result(input: f64, in_min: f64, in_max: f64, out_min: f64, out_max: f64) {
	let result = remap(&input, &in_min, &in_max, &out_min, &out_max);
	
	// columns/rows of things to show
	let in_min = in_min.to_string();
	let out_min = out_min.to_string();
	
	let in_max = in_max.to_string();
	let out_max = out_max.to_string();
	
	// finding how wide each column is
	let min_col_width = max(in_min.chars().count(), out_min.chars().count());
	let max_col_width = max(in_max.chars().count(), out_max.chars().count());
	
	// add spaces
	let in_min = in_min.pad(min_col_width, ' ', pad::Alignment::Right, false);
	let out_min = out_min.pad(min_col_width, ' ', pad::Alignment::Right, false);
	
	let in_max = in_max.pad(max_col_width, ' ', pad::Alignment::Right, false);
	let out_max = out_max.pad(max_col_width, ' ', pad::Alignment::Right, false);
	
	println!("Input: {input}");
	println!("");
	println!(" In: {in_min} .. {in_max}");
	println!("Out: {out_min} .. {out_max}");
	println!("");
	println!("= {result}");
}


fn remap(input: &f64, in_min: &f64, in_max: &f64, out_min: &f64, out_max: &f64) -> f64{
	out_min + (input-in_min) / (in_max-in_min) * (out_max-out_min)
}