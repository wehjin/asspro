use serde::{Deserialize, Serialize};
use crate::core::Event;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct State {
	pub head: Option<Event>,
	pub pins: Vec<String>,
}

impl Default for State {
	fn default() -> Self {
		State { head: None, pins: Vec::new() }
	}
}

impl State {
	pub fn add_pin(&self, hash: &String) -> Self {
		let mut state = self.clone();
		if !state.pins.contains(hash) {
			state.pins.push(hash.clone());
		}
		state
	}
	pub fn set_head(&self, event: Event) -> Self {
		let mut state = self.clone();
		state.head = Some(event);
		state
	}
}
