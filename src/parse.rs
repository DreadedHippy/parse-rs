
pub fn parse(input_string: String){
	// Begin parsing
	let mut output_string = String::new();
	let mut format_stack: Vec<String> = Vec::new();

	let input_lines = input_string.split("\n").map(String::from).collect::<Vec<String>>();

	let mut link_holder: [Option<usize>; 4] = [None, None, None, None];
	let mut img_holder: [Option<usize>; 4] = [None, None, None, None];
	


	for mut line in input_lines {
		line = line.trim().to_string();
		// println!("{:?}", line); // "# Test for h1 /r"
		if line.starts_with("# ") {
			output_string.push_str("<h1>");
			line = line.chars().skip(2).collect();
			parse_line(line, &mut output_string, &mut format_stack, &mut link_holder, &mut img_holder);
			output_string.push_str("</h1>\n");

		} else if line.starts_with("## ") {
			output_string.push_str("<h2>");
			line = line.chars().skip(3).collect();
			parse_line(line, &mut output_string, &mut format_stack, &mut link_holder, &mut img_holder);
			output_string.push_str("</h2>\n");

		} else if line.starts_with("### ") {
			output_string.push_str("<h3>");
			line = line.chars().skip(4).collect();
			parse_line(line, &mut output_string, &mut format_stack, &mut link_holder, &mut img_holder);
			output_string.push_str("</h3>\n");

		} else if line.starts_with("#### ") {
			output_string.push_str("<h4>");
			line = line.chars().skip(5).collect();
			parse_line(line, &mut output_string, &mut format_stack, &mut link_holder, &mut img_holder);
			output_string.push_str("</h4>\n");

		} else if line.starts_with("##### ") {
			output_string.push_str("<h5>");
			line = line.chars().skip(6).collect();
			parse_line(line, &mut output_string, &mut format_stack, &mut link_holder, &mut img_holder);
			output_string.push_str("</h5>\n");

		} else if line.starts_with("###### ") {
			output_string.push_str("<h6>");
			line = line.chars().skip(7).collect();
			parse_line(line, &mut output_string, &mut format_stack, &mut link_holder, &mut img_holder);
			output_string.push_str("</h6>\n");

		} else if line.starts_with("- ") {
			if format_stack.last() != Some(&"- ".to_string()) {
				format_stack.push("- ".to_string());
				output_string.push_str("<ul>\n");
			}
			output_string.push_str("\t<li>");
			line = line.chars().skip(2).collect();
			parse_line(line, &mut output_string, &mut format_stack, &mut link_holder, &mut img_holder);
			output_string.push_str("</li>\n");

		} else if line == "---".to_string() {
			output_string.push_str("\n<hr/>\n");

		} else if line == "".to_string() {
			output_string.push_str("\n");

		} else {
			if format_stack.last() == Some(&"- ".to_string()) {
				format_stack.pop();
				output_string.push_str("</ul>\n");
			}
			parse_line(line, &mut output_string, &mut format_stack, &mut link_holder, &mut img_holder);
		}
	}

	if format_stack.last() == Some(&"p".to_string()){
		format_stack.pop();
		output_string.push_str("</p>");
	}

	// println!("{:?}", format_stack);
	println!("{}", output_string);
}

fn parse_line(input: String, output_string: &mut String, format_stack: &mut Vec<String>, link_holder: &mut [Option<usize>; 4], img_holder: &mut [Option<usize>; 4]) {

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
			'!' => {
				if input_as_vec[index+1] == '[' {
					format_stack.push("![".to_string());
					img_holder[0] = Some(index+1);
					index+=1;
				}

				// println!("{} {}", underline_string, index)
			},
			'[' => {
				format_stack.push("[".to_string());
				link_holder[0] = Some(index);

				// println!("{} {}", underline_string, index)
			},
			']' => {
				if format_stack.last() == Some(&"[".to_string()) {
					if input_as_vec[index+1] == '(' {
						format_stack.pop();
						format_stack.push("(".to_string());
						link_holder[1] = Some(index);
						link_holder[2] = Some(index+1);
						index+=1;
					}
				} else if format_stack.last() == Some(&"![".to_string()) {
					if input_as_vec[index+1] == '(' {
						format_stack.pop();
						format_stack.push("!(".to_string());
						img_holder[1] = Some(index);
						img_holder[2] = Some(index+1);
						index+=1;
					}
				}
				// println!("{} {}", underline_string, index)
			},
			')' => {
				if format_stack.last() == Some(&"(".to_string()) {
					format_stack.pop();
					link_holder[3] = Some(index);

					if link_holder.iter().all(|x| x != &None) {
						let href_distance = link_holder[3].unwrap() - 1 - link_holder[2].unwrap();
						let title_distance = link_holder[1].unwrap() - 1 - link_holder[0].unwrap();

						// eprintln!("TITLE_DISTANCE: {}, HREF_DISTANCE: {}", title_distance, href_distance);


						let characters = output_string.chars().rev().take(href_distance+title_distance).collect::<Vec<char>>();
						let characters = characters.into_iter().rev().collect::<Vec<char>>();
						output_string.truncate(output_string.len() - (href_distance+title_distance));
						let title = characters[0..title_distance].to_vec().iter().collect::<String>();
						let href = characters[title_distance..href_distance+title_distance].to_vec().iter().collect::<String>();

						// eprintln!("CHARACTERS: {:?}", characters);

						let link_string = format!("<a href=\"{}\">{}</a>", href, title);

						// println!("LINK STRING: {}", link_string);

						output_string.push_str(&link_string);
						link_holder[0] = None;
						link_holder[1] = None;
						link_holder[2] = None;
						link_holder[3] = None;

					}

				} else if format_stack.last() == Some(&"!(".to_string()) {
					format_stack.pop();
					img_holder[3] = Some(index);

					if img_holder.iter().all(|x| x != &None) {
						let img_distance = img_holder[3].unwrap() - 1 - img_holder[2].unwrap();
						let alt_distance = img_holder[1].unwrap() - 1 - img_holder[0].unwrap();

						// eprintln!("TITLE_DISTANCE: {}, HREF_DISTANCE: {}", title_distance, href_distance);


						let characters = output_string.chars().rev().take(img_distance+alt_distance).collect::<Vec<char>>();
						let characters = characters.into_iter().rev().collect::<Vec<char>>();
						output_string.truncate(output_string.len() - (img_distance+alt_distance));
						let alt = characters[0..alt_distance].to_vec().iter().collect::<String>();
						let img = characters[alt_distance..img_distance+alt_distance].to_vec().iter().collect::<String>();

						// eprintln!("CHARACTERS: {:?}", characters);

						let image_string = format!("<img src=\"{}\" alt=\"{}\" />", img, alt);

						// println!("LINK STRING: {}", link_string);

						output_string.push_str(&image_string);
						img_holder[0] = None;
						img_holder[1] = None;
						img_holder[2] = None;
						img_holder[3] = None;

					}

				}
				// println!("{} {}", underline_string, index)
			},
			
			_ => output_string.push(character)
		}

		index+=1;
	}

}
