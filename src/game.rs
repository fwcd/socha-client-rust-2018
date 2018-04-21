use std::string::String;

pub struct Room {
	pub id: String
}

pub struct Joined {
	pub id: String
}

pub struct WelcomeMessage {
	pub color: String
}

pub struct Card {
	pub card_type: String
}

impl Clone for Card {
	fn clone(&self) -> Self {
		return Card {
			card_type: self.card_type.clone()
		};
	}
}

pub struct Player {
	pub display_name: String,
	pub color: String,
	pub index: i32,
	pub carrots: i32,
	pub salads: i32,
	pub cards: Vec<Card>
}

impl Clone for Player {
	fn clone(&self) -> Self {
		return Player {
			display_name: self.display_name.clone(),
			color: self.color.clone(),
			index: self.index,
			carrots: self.index,
			salads: self.salads,
			cards: self.cards.clone()
		};
	}
}

pub struct Move {
	pub xml_move: String
}

pub struct GameState {
	pub red_player: Player,
	pub blue_player: Player,
	pub board: Board
}

pub struct Memento {
	pub state: GameState
}

pub struct Field {
	pub field_type: String,
	pub index: i32
}

pub struct Board {
	pub fields: Vec<Field>
}