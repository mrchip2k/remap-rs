use colored::*;
use crate::common::*;

const ROW_COUNT: usize = 3;

pub fn display_result(result: NumType, arg_st: ArgStruct) {
//// Create columns
	let mut columns: Vec<Column> = Vec::new();
	let col_explain_input = Column {
		contents: [
			"        ".to_string(),
			"Input → ".bold().to_string(),
			"        ".to_string()
		],
		width: 8
	};
	let col_explain_output = Column {
		contents: [
			"         ".to_string(),
			" ← Output".bold().to_string(),
			"         ".to_string()
		],
		width: 9
	};
	let col_middle = Column {
		contents: [
			" min ".to_string(),
			" --> ".bold().to_string(),
			" max ".to_string()
		],
		width: 5
	};
	let mut col_input = Column {
		contents: [
			rounded_display(arg_st.in_min),
			rounded_display(arg_st.input),
			rounded_display(arg_st.in_max)
		],
		width: 0 // unknown
	};
	col_input.calc_width();
	let mut col_output = Column {
		contents: [
			rounded_display(arg_st.out_min),
			result.to_string(),
			rounded_display(arg_st.out_max)
		],
		width: 0 // unknown
	};
	col_output.calc_width();
//// Padding on data columns
	for i in 0..ROW_COUNT {
		let mut new = String::new();
		let len = col_input.contents[i].chars().count();
		let missing = col_input.width - len;
		for _ in 0..missing {
			new.push(' ');
		}
		new.push_str(&col_input.contents[i]);
		col_input.contents[i] = new;
	}
	for i in 0..ROW_COUNT {
		let mut new = String::new();
		new.push_str(&col_output.contents[i]);
		let len = col_output.contents[i].chars().count();
		let missing = col_output.width - len;
		for _ in 0..missing {
			new.push(' ');
		}
		col_output.contents[i] = new;
	}
//// Bold on data columns
	// this HAS to be done after counting the width,
	// or else the style control characters get counted
	// and it messes up the other rows alignment.
	col_input.contents[1] = col_input.contents[1].bold().to_string();
	col_output.contents[1] = col_output.contents[1].bold().to_string();
//// Assembling the table
	columns.push(col_explain_input);
	columns.push(col_input);
	columns.push(col_middle);
	columns.push(col_output);
	columns.push(col_explain_output);
//// Rendering the table
	let mut print_buf = String::new();
	for i in 0..ROW_COUNT {
		for col in &columns {
			print_buf.push_str(&col.contents[i]);
		}
		print_buf.push('\n');
	}
	println!("{}", print_buf);
}

struct Column {
	contents: [String; ROW_COUNT],
	width: usize,
}
impl Column {
	fn calc_width(&mut self) {
		for row in &self.contents {
			let this_len = row.chars().count();
			if this_len > self.width {
				self.width = this_len;
			}
		}
	}
}

fn rounded_display (num: NumType) -> String {
	let rounded = (num * 100.0).round() / 100.0;
	let mut rounded_str = rounded.to_string();
	if num != rounded {
		rounded_str.push_str("..");
	}
	rounded_str
}
