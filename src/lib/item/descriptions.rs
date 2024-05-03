/// All the names and descriptions for items.
pub const ITEMLORE: [[&str; 2]; 5] = [
    ["Basic Staff", "A staff that can be found anywhere. Nothing special."], 
    ["Basic Health Potion", "The cheapest of healing potions."],
    ["Basic Mage Robe", "As much use as a bath robe."],
    ["Your Bed", "The place you slept in for the past 6 years. That changes today."],
    ["Mirror", "It refects your beautiful face."],
    ];
/// All the names and descriptions for rooms.
pub const ROOMLORE: [[&str; 2]; 3] = [
    // TODO! find a way to print text based on updated information
    // example: you pick up the staff and the text giving notice to the staff goes away
    // when you enter the room again
    ["Bedroom", "The dorm looks clean but empty. Seems like the owner isn't home often."],
    ["Bathroom", "A small dorm bathroom."],
    ["Hallway", "The unkept hallway that connects school dorms together."]
];
    /// Holds the clues a room has.
pub const SEARCHLORE: [[&str; 1]; 3] = [
    ["There is a staff by the bed. The only doors lead to the bathroom and the hallway."],
    ["This is a bathroom connected to a bedroom. It houses a giant mirror."],
    ["Doors connecting to many dorm rooms line this hallway.
Leaving the hallway leads to the rest of the school dorms."]    
];

/// Get the descriptions or name of an item.
pub fn get_item_lore(index1: usize, index2: usize) -> &'static str {
    ITEMLORE[index1][index2]
}
/// Get the descriptions or name of the room.
pub fn get_room_lore(index1: usize, index2: usize) -> &'static str {
    ROOMLORE[index1][index2]
}