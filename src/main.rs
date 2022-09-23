use std::{env::{self}};
use colored::*;
mod display_result;
use display_result::*;

fn main() {
	let args = get_args();
	match args {
		None => {
			println!("{}", "Pass arguments to calculate, in this order:".bold());
			println!("{}, {}, {}, {}, {}.",
				"input".purple(),
				"in min".red(),
				"in max".yellow(),
				"out min".green(),
				"out max".blue()
			);
			println!("Example:");
			println!("remap {} {} {} {} {}",
				"0.75".purple(),
				"0".red(),
				"1".yellow(),
				"-100".green(),
				"100".blue()
			);
			println!("Result: 50");
		},
		Some(arg_st) => {
			let result = remap(
				&arg_st.input,
				&arg_st.in_min,
				&arg_st.in_max,
				&arg_st.out_min,
				&arg_st.out_max
			);
			display_result(result, arg_st);
		}
	}
}

pub struct ArgStruct {
	input: f64,
	in_min: f64,
	in_max: f64,
	out_min: f64,
	out_max: f64
}

fn get_args() -> Option<ArgStruct> {
	let args_str: Vec<String> = env::args().collect();
	if args_str.len() != 6 {
		return None;
	}
	let mut args_float: Vec<f64> = Vec::new();
	for arg in args_str[1..].iter() {
		let parsed = arg.parse::<f64>();
		if parsed.is_err() {
			return None;
		}
		args_float.push(parsed.unwrap());
	}
	return Some(ArgStruct {
		input: args_float[0],
		in_min: args_float[1],
		in_max: args_float[2],
		out_min: args_float[3],
		out_max: args_float[4]
	});
}

fn remap(input: &f64, in_min: &f64, in_max: &f64, out_min: &f64, out_max: &f64) -> f64{
	out_min + (input-in_min) / (in_max-in_min) * (out_max-out_min)
}
