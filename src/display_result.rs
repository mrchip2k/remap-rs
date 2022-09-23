use crate::ArgStruct;
use colored::*;

const ROW_COUNT: usize = 3;

pub fn display_result(result: f64, arg_st: ArgStruct) {
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
			arg_st.in_min.to_string(),
			arg_st.input.to_string(),
			arg_st.in_max.to_string()
		],
		width: 0 // unknown
	};
	col_input.calc_width();
	let mut col_output = Column {
		contents: [
			arg_st.out_min.to_string(),
			result.to_string(),
			arg_st.out_max.to_string()
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
	for i in 0..ROW_COUNT {
		let mut line_buf = String::new();
		for col in &columns {
			line_buf.push_str(&col.contents[i]);
		}
		println!("{}", line_buf);
	}
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
