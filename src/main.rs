use std::{
	collections::HashMap,
	io,
};
use svg::{
	Document,
	node::element::Path,
	node::element::path::Data,
};

use LindenmeyerSystemPreset::*;

enum Action {
	DecrementAngle,
	Forward,
	IncrementAngle,
	None,
	Pop,
	Push,
}

#[derive(Clone, Copy)]
enum LindenmeyerSystemPreset {
	HeighwayDragon,
	Plant,
	Tree,
}

struct LindenmeyerSystem {
	actions: HashMap<char, Action>,
	angle_delta: f32,
	axiom: String,
	rules: HashMap<char, String>,
}

impl LindenmeyerSystem {
	fn recommended_generations(preset: LindenmeyerSystemPreset) -> u8 {
		match preset {
			HeighwayDragon => 16,
			Plant => 6,
			Tree => 7,
		}
	}

	/// Uses a LindenmeyerSystemPreset to return values known to produce something cool. See the Wikipedia page "L-System" for more information.
	fn from_preset(preset: LindenmeyerSystemPreset) -> Self {
		let angle_delta;
		let axiom;
		let mut actions = HashMap::new();
		let mut rules = HashMap::new();
		match preset {
			HeighwayDragon => {
				actions.insert('F', Action::Forward);
				actions.insert('G', Action::Forward);
				actions.insert('+', Action::IncrementAngle);
				actions.insert('-', Action::DecrementAngle);
				angle_delta = std::f32::consts::PI / 2.0;
				axiom = String::from("F");
				rules.insert('F', String::from("F+G"));
				rules.insert('G', String::from("F-G"));
				rules.insert('+', String::from("+"));
				rules.insert('-', String::from("-"));
			},
			Plant => {
				actions.insert('X', Action::None);
				actions.insert('F', Action::Forward);
				actions.insert('+', Action::IncrementAngle);
				actions.insert('-', Action::DecrementAngle);
				actions.insert('[', Action::Push);
				actions.insert(']', Action::Pop);
				angle_delta = (5.0 * std::f32::consts::PI) / 36.0; // 25 degrees
				axiom = String::from("X");
				rules.insert('X', String::from("F+[[X]-X]-F[-FX]+X"));
				rules.insert('F', String::from("FF"));
				rules.insert('+', String::from("+"));
				rules.insert('-', String::from("-"));
				rules.insert('[', String::from("["));
				rules.insert(']', String::from("]"));
			},
			Tree => {
				actions.insert('0', Action::Forward);
				actions.insert('1', Action::Forward);
				actions.insert('l', Action::IncrementAngle);
				actions.insert('r', Action::DecrementAngle);
				actions.insert('[', Action::Push);
				actions.insert(']', Action::Pop);
				angle_delta = std::f32::consts::PI / 6.0;
				axiom = String::from("0");
				rules.insert('0', String::from("1[l0]r0"));
				rules.insert('1', String::from("11"));
				rules.insert('l', String::from("l"));
				rules.insert('r', String::from("r"));
				rules.insert('[', String::from("["));
				rules.insert(']', String::from("]"));
			},
		}
		Self {
			actions,
			angle_delta,
			axiom,
			rules,
		}
	}
}


fn main() {
	eprintln!("select a preset by pressing its character:");
	eprintln!("\t[h] Heighway Dragon");
	eprintln!("\t[p] Plant");
	eprintln!("\t[t] Tree");
	let mut input_buffer = String::new();
	let stdin = io::stdin();
	let selection;
	loop {
		stdin.read_line(&mut input_buffer).unwrap();
		selection = match input_buffer.trim() {
			// TODO: implement custom input
			"h" => HeighwayDragon,
			"p" => Plant,
			"t" => Tree,
			_ => {
				eprintln!("unrecognized. reinput:");
				input_buffer.clear();
				continue;
			}
		};
		break;
	}
	let generations = LindenmeyerSystem::recommended_generations(selection);
	let system = LindenmeyerSystem::from_preset(selection);

	let angle_delta = system.angle_delta;
	let mut angle = 0.0;
	let mut string = system.axiom;
	let mut data = Data::new().move_to((0, 0));
	let mut position = (0, 0);
	let mut stack = Vec::new();

	for _ in 0..generations {
		let mut string_new = String::new();
		for c in string.chars() {
			let replacement = &system.rules[&c];
			string_new.push_str(replacement);
		}
		string = string_new;
	}

	for c in string.chars() {
		match system.actions[&c] {
			Action::DecrementAngle => angle -= angle_delta,
			Action::Forward => {
				let (new_x, new_y) = get_vector(angle, 10);
				let (old_x, old_y) = position;
				position = (old_x + new_x, old_y + new_y);
				data = data.line_by((new_x, new_y));
			},
			Action::IncrementAngle => angle += angle_delta,
			Action::None => {},
			Action::Pop => {
				(position, angle) = stack.pop().expect("malformed system");
				data = data.move_to(position);
			},
			Action::Push => stack.push((position, angle)),
		}
	}

	let path = Path::new()
		.set("fill", "none")
		.set("stroke", "black")
		.set("stroke-width", 3)
		.set("d", data);

	let document = Document::new()
		.set("viewBox", (-1000, -1000, 2000, 2000))
		.add(path);

	// this program writes the svg straight to standard output. redirect!
	let stdout = io::stdout();
	svg::write(stdout, &document).unwrap();
}

fn get_vector(angle: f32, length: isize) -> (isize, isize) {
	let length = length as f32;
	((length * angle.cos()) as isize, (length * angle.sin()) as isize)
}
