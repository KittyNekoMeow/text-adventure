use crate::{area::def::Room, gamestate::def::GameState};

use super::def::print_interactable;
// TODO! fix matching of the words
// get itemioejfm;aefjipe picks up the item
pub const COLLECTKEYWORDS: [&str; 3] = ["get ", "collect ", "pick up "];
pub const EXAMAINEKEYWORDS: [&str; 4] = ["check ", "examine ", "look at ", "inspect "];
/// Prints the interactable item or confirms the ability to pick up an item.
pub fn item_interaction(input: &String, gamestate: &mut GameState) -> i8 {
    for i in COLLECTKEYWORDS {
        if input.as_str().contains(i) {
            return 1;
        }
    }
    for i in EXAMAINEKEYWORDS {
        if input.as_str().contains(i) {
        if check_int_item_field(input, &gamestate.current_area.room) {
        print_interactable(gamestate.current_area.room.interactable_items[gamestate.get_inter_index(input)].1.desc_id);
            return 0;
            }
        }
    };
    return 3;
}

/// Checks which field you are looking for
fn check_int_item_field(input: &String, room_items: &Room) -> bool {
    for i in &room_items.interactable_items {
        if input.contains(&i.0) {
            return true;
        }
    }
    return false;
}