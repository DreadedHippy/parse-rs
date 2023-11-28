
pub fn parse(input_string: String){
	// Begin parsing
	let mut output_string = String::new();
	let mut format_stack: Vec<String> = Vec::new();

	let input_lines = input_string.split("\n").map(String::from).collect::<Vec<String>>();
	


	for mut line in input_lines {
		line = line.trim().to_string();
		// println!("{:?}", line); // "# Test for h1 /r"
		if line.starts_with("# ") {
			output_string.push_str("<h1>");
			line = line.chars().skip(2).collect();
			parse_line(line, &mut output_string, &mut format_stack);
			output_string.push_str("</h1>\n");

		} else if line.starts_with("## ") {
			output_string.push_str("<h2>");
			line = line.chars().skip(3).collect();
			parse_line(line, &mut output_string, &mut format_stack);
			output_string.push_str("</h2>\n");

		} else if line.starts_with("### ") {
			output_string.push_str("<h3>");
			line = line.chars().skip(4).collect();
			parse_line(line, &mut output_string, &mut format_stack);
			output_string.push_str("</h3>\n");

		} else if line.starts_with("#### ") {
			output_string.push_str("<h4>");
			line = line.chars().skip(5).collect();
			parse_line(line, &mut output_string, &mut format_stack);
			output_string.push_str("</h4>\n");

		} else if line.starts_with("##### ") {
			output_string.push_str("<h5>");
			line = line.chars().skip(6).collect();
			parse_line(line, &mut output_string, &mut format_stack);
			output_string.push_str("</h5>\n");

		} else if line.starts_with("###### ") {
			output_string.push_str("<h6>");
			line = line.chars().skip(7).collect();
			parse_line(line, &mut output_string, &mut format_stack);
			output_string.push_str("</h6>\n");

		} else if line.starts_with("- ") {
			if format_stack.last() != Some(&"- ".to_string()) {
				format_stack.push("- ".to_string());
				output_string.push_str("<ul>\n");
			}
			output_string.push_str("\t<li>");
			line = line.chars().skip(2).collect();
			parse_line(line, &mut output_string, &mut format_stack);
			output_string.push_str("</li>\n");

		} else {
			if format_stack.last() == Some(&"- ".to_string()) {
				format_stack.pop();
				output_string.push_str("</ul>\n");
			}
			parse_line(line, &mut output_string, &mut format_stack);
		}
	}

	if format_stack.last() == Some(&"p".to_string()){
		format_stack.pop();
		output_string.push_str("</p>");
	}

	// println!("{:?}", format_stack);
	println!("{}", output_string);
}

fn parse_line(input: String, output_string: &mut String, format_stack: &mut Vec<String>) {

	let input_as_vec = input.chars().collect::<Vec<char>>();

	let mut index = 0;
	while index < input_as_vec.len(){

		let character = input_as_vec[index];

		match character {
			'*' => {
				let mut star_string = String::new();
				while index < input_as_vec.len() && input_as_vec[index] == '*' && star_string.len() < 3 {
					star_string.push('*');
					index+=1;
				}
				index-=1;

				if format_stack.last() == Some(&star_string) {
					format_stack.pop();
					match star_string.as_str() {
						"*" => {output_string.push_str("</i>")},
						"**" => {output_string.push_str("</b>")},
						"***" => {output_string.push_str("</i></b>")},
						_ => {}
					}
				} else {
					format_stack.push(star_string.clone());
					match star_string.as_str() {
						"*" => {output_string.push_str("<i>")},
						"**" => {output_string.push_str("<b>")},
						"***" => {output_string.push_str("<b><i>")},
						_ => {}
					}

				}

				// println!("{} {}", star_string, index)
			},
			'_' => {
				let mut underline_string = String::new();
				while index < input_as_vec.len() && input_as_vec[index] == '_' {
					underline_string.push('_');
					index+=1;
				}
				index-=1;

				if format_stack.last() == Some(&underline_string) {
					format_stack.pop();
					match underline_string.as_str() {
						"__" => {
							output_string.push_str("</u>")
						},
						_ => {}
					}
				} else {
					format_stack.push(underline_string.clone());
					match underline_string.as_str() {
						"__" => {
							output_string.push_str("<u>")
						},
						_ => {}
					}

				}

				// println!("{} {}", underline_string, index)
			},
			'`' => {
				if format_stack.last() == Some(&"`".to_string()) {
					format_stack.pop();
					output_string.push_str("</code>");
				} else {
					format_stack.push("`".to_string());
					output_string.push_str("<code>")
				}

				// println!("{} {}", underline_string, index)
			}
			
			_ => output_string.push(character)
		}

		index+=1;
	}

}
