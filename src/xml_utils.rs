extern crate xml;

use std::collections::HashMap;
use std::collections::VecDeque;
use std::vec::Vec;
use std::io::BufReader;
use std::net::TcpStream;
use self::xml::reader::{EventReader, XmlEvent};
use super::game::*;

#[derive(Debug)]
pub struct XMLNode {
	name: String,
	attribs: HashMap<String, Vec<String>>,
	childs: Vec<XMLNode>
}

impl XMLNode {
	fn new() -> XMLNode {
		return XMLNode { name: String::new(), attribs: HashMap::new(), childs: Vec::new() }
	}

	pub fn read_from(xml_parser: &mut EventReader<BufReader<&TcpStream>>) -> XMLNode {
		let mut node_stack: VecDeque<XMLNode> = VecDeque::new();
		node_stack.push_back(XMLNode::new());

		loop {
			match xml_parser.next() {
				Ok(XmlEvent::StartElement { name, attributes, .. }) => {
					println!("Received {} with {:?}", name, attributes); // DEBUG
					let mut node = node_stack.pop_back().expect("Unexpectedly found empty XML node stack after receiving new start element.");
					node.name = name.local_name;
					for attribute in attributes {
						let attrib_name = attribute.name.local_name;
						if !node.attribs.contains_key(&attrib_name) {
							node.attribs.insert(attrib_name.to_string(), Vec::new());
						}
						node.attribs.get_mut(&attrib_name).unwrap().push(attribute.value.to_string());
					}
					node_stack.push_back(node);
				},
				Ok(XmlEvent::EndElement { .. }) => {
					if node_stack.len() > 2 {
						let child = node_stack.pop_back().expect("Unexpectedly found empty XML node stack while trying to pop off new child element");
						let mut node = node_stack.pop_back().expect("Unexpectedly found empty XML node stack while trying to hook up new child element");
						node.childs.push(child);
						node_stack.push_back(node);
					}
				},
				Err(e) => {
					println!("XMLNode error: {}", e);
					break;
				},
				_ => {}
			}

			// Condition
			if node_stack.len() <= 1 { break; }
		}

		return node_stack.pop_back().expect("Unexpectedly found empty XML node stack while trying to return nodes.");
	}

	pub fn as_game_state(&self) -> GameState {
		let err = "Error while parsing XML node to GameState";
		return GameState {
			red_player: self.get_child("red").expect(err).as_player(),
			blue_player: self.get_child("blue").expect(err).as_player(),
			board: self.get_child("board").expect(err).as_board()
		};
	}

	pub fn as_player(&self) -> Player {
		let err = "Error while parsing XML node to Player";
		return Player {
			display_name: self.get_attribute("displayName").expect(err).to_string(),
			color: self.get_attribute("color").expect(err).to_string(),
			index: self.get_attribute("index").expect(err).parse::<i32>().expect(err),
			carrots: self.get_attribute("carrots").expect(err).parse::<i32>().expect(err),
			salads: self.get_attribute("salads").expect(err).parse::<i32>().expect(err),
			cards: self.get_child_vec("cards").iter().map(|node| node.as_card()).collect()
		};
	}

	pub fn as_room(&self) -> Room {
		let err = "Error while parsing XML node to Room";
		return Room {
			id: self.get_attribute("roomId").expect(err).to_string()
		};
	}

	pub fn as_joined(&self) -> Joined {
		let err = "Error while parsing XML node to Joined";
		return Joined {
			id: self.get_attribute("roomId").expect(err).to_string()
		};
	}

	pub fn as_welcome_message(&self) -> WelcomeMessage {
		let err = "Error while parsing XML node to WelcomeMessage";
		return WelcomeMessage {
			color: self.get_attribute("color").expect(err).to_string()
		};
	}

	pub fn as_card(&self) -> Card {
		let err = "Error while parsing XML node to Card";
		return Card {
			card_type: self.get_attribute("type").expect(err).to_string()
		};
	}

	pub fn as_board(&self) -> Board {
		return Board {
			fields: self.get_child_vec("fields").iter().map(|n| n.as_field()).collect()
		};
	}

	pub fn as_memento(&self) -> Memento {
		println!("Memento: {:?}", self); // DEBUG
		let err = "Error while parsing XML node to Memento";
		return Memento {
			state: self.get_child("state").expect(err).as_game_state()
		};
	}

	pub fn as_field(&self) -> Field {
		let err = "Error while parsing XML node to Field";
		return Field {
			field_type: self.get_attribute("type").expect(err).to_string(),
			index: self.get_attribute("index").expect(err).parse::<i32>().expect(err)
		};
	}
	
	pub fn get_name(&self) -> &String { return &self.name; }

	pub fn get_attributes(&self) -> &HashMap<String, Vec<String>> { return &self.attribs; }

	pub fn get_attribute(&self, name: &str) -> Option<&String> { return self.attribs.get(name).map(|a| &a[0]); }

	pub fn get_child_vec(&self, name: &str) -> Vec<&XMLNode> {
		let mut result: Vec<&XMLNode> = Vec::new();

		for child in &self.childs {
			if child.name.as_str() == name {
				result.push(&child);
			}
		}

		return result;
	}

	pub fn get_childs(&self) -> &Vec<XMLNode> { return &self.childs; }

	pub fn get_child(&self, name: &str) -> Option<&XMLNode> {
		for child in &self.childs {
			if child.name.as_str() == name {
				return Some(&child);
			}
		}

		return None;
	}
}

impl Clone for XMLNode {
	fn clone(&self) -> Self {
		return XMLNode {
			name: self.name.clone(),
			attribs: self.attribs.clone(),
			childs: self.childs.clone()
		};
	}
}