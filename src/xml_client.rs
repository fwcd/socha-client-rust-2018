extern crate xml;

use std::net::TcpStream;
use std::io::{BufReader, BufWriter, prelude::Write};
use self::xml::reader::*;
use super::game::*;
use super::xml_utils::XMLNode;

pub struct XMLClient {
	listeners: Vec<Box<ClientListener>>,
	my_color: Option<String>,
	game_state: Option<GameState>,
	room: Option<Room>
}

impl XMLClient {
	pub fn new() -> XMLClient {
		return XMLClient {
			listeners: Vec::new(),
			my_color: None,
			game_state: None,
			room: None
		};
	}

	/**
	 * Runs (and consumes) the client.
	 */
	pub fn run(mut self, target: &String, reservation: &String) {
		println!("Connecting to {}...", target);
		let stream = TcpStream::connect(target).expect("Could not connect to server");

		println!("Connected to {}", &target);
		XMLClient::write_to(&stream, "<protocol>");
		
		let join_xml: String;
		match reservation.as_str() {
			"" => join_xml = "<join gameType=\"swc_2018_hase_und_igel\"/>".to_string(),
			_ => join_xml = format!("<joinPrepared reservationCode=\"{}\" />", reservation)
		}

		println!("Sending join message: {}", join_xml);
		XMLClient::write_to(&stream, join_xml.as_str());

		self.handle_stream(&stream);
	}

	fn fire_listeners(&mut self, notifier: &mut FnMut(&mut ClientListener)) {
		let length = self.listeners.len();
		for i in 0..length {
			let mut boxed: &mut Box<ClientListener> = &mut self.listeners[i];
			let mut reference = boxed.as_mut();
			notifier(reference);
		}
	}

	fn handle_stream(mut self, stream: &TcpStream) {
		let mut parser = EventReader::new(BufReader::new(stream));

		loop {
			let mut node = XMLNode::read_from(&mut parser);
			match node.get_name().as_str() {
				"data" => {
					let invalid = &"".to_string();
					let data_class = node.get_attribute("class").unwrap_or(invalid).to_string();
					match data_class.as_str() {
						"memento" => self.handle_memento_node(&mut node),
						"welcomeMessage" => self.handle_welcome_message_node(&mut node),
						"sc.framework.plugins.protocol.MoveRequest" => {
							let color = self.my_color.iter().clone().last().expect("Could not find player color.").as_str();
							let mut move_req_listener = &mut *self.listeners[0];
							let game_state = &self.game_state.iter().clone().last().expect("Could not find current game state.");
							let xml_move = XMLClient::get_move_upon_request(color, move_req_listener, game_state, &mut node).xml_move;
							XMLClient::write_to(stream, &format!("<room roomId=\"%s\"><data class=\"move\">{}</data></room>", xml_move));
						},
						_ => {}
					}
				},
				"joined" => self.handle_joined_node(&mut node),
				_ => {}
			}
		}
	}

	fn handle_joined_node(&mut self, node: &mut XMLNode) {
		let room = node.as_room();
		self.fire_listeners(&mut |listener| listener.on_join(&room));
		self.room = Some(room);
	}

	fn handle_memento_node(&mut self, node: &mut XMLNode) {
		let memento = node.as_memento();
		self.fire_listeners(&mut |listener| listener.on_update_state(&memento.state));
		self.game_state = Some(memento.state);
	}

	fn handle_welcome_message_node(&mut self, node: &mut XMLNode) {
		let msg = node.as_welcome_message();
		self.fire_listeners(&mut |listener| listener.on_welcome_message(&msg));
		self.my_color = Some(msg.color.to_string());
	}

	fn get_move_upon_request(color: &str, move_req_listener: &mut ClientListener, game_state: &GameState, node: &mut XMLNode) -> Move {
		let me: &Player;
		let opponent: &Player;

		match color {
			"red" => {
				me = &game_state.red_player;
				opponent = &game_state.blue_player;
			},
			_ => {
				opponent = &game_state.red_player;
				me = &game_state.blue_player;
			}
		}

		return move_req_listener.on_move_request(game_state, me, opponent);
	}

	fn write_to(stream: &TcpStream, data: &str) {
		let _ = BufWriter::new(stream).write(data.as_bytes());
	}
}

pub trait ClientListener {
	fn on_update_state(&mut self, state: &GameState) {}

	fn on_welcome_message(&mut self, welcome_message: &WelcomeMessage) {}

	fn on_move_request(&mut self, state: &GameState, me: &Player, opponent: &Player) -> Move {
		// Use simple logic by default
		let mut next_carrot_field_index: Option<i32> = None;
		let fields = &state.board.fields;

		for (i, field) in fields.iter().enumerate() {
			let index = i as i32;
			if index > me.index && opponent.index != index && field.field_type.as_str() == "CARROT" {
				next_carrot_field_index = Some(index);
				break;
			}
		}

		let carrot_field = next_carrot_field_index.expect("Could not find next carrot field.");
		let distance = carrot_field - me.index;

		if ((distance * (distance + 1)) / 2) > me.carrots {
			// Not enough carrots to move
			return Move { xml_move: "<fallBack order=\"0\" />".to_string() };
		} else {
			return Move { xml_move: format!("<advance order=\"0\" distance=\"{}\" />", distance) };
		}
	}

	fn on_join(&mut self, room: &Room) {}
}