use std::io::Write;
use crate::things::thing::Thing;

mod game;
mod globals;
mod things;

fn main() {
    let treasure_ring = things::treasure::Treasure::new("ring".to_owned(), "It is a ring of great power".to_owned(), 500);
    let treasure_wombat = things::treasure::Treasure::new("wombat".to_owned(), "It is a cuddly little wombat. It is squeaking gently to itself.".to_owned(), 700);
    let dungeon_list: Vec<Box<dyn Thing>> = vec![Box::new(treasure_ring), Box::new(treasure_wombat)];
    let room_1 = things::room::Room::new("Troll Room".to_owned(), "A dank room that smells of troll".to_owned(), vec![], -1, 2, -1, 1);
    let room_2 = things::room::Room::new("Forest".to_owned(), "A leafy woodland".to_owned(), vec![], -1, -1, 0, -1);
    let room_3 = things::room::Room::new("Cave".to_owned(), "A dismal cave with walls covered in luminous moss".to_owned(), vec![], 0, -1, -1, 3);
    let room_4 = things::room::Room::new("Dungeon".to_owned(), "A nasty, dark cell".to_owned(), dungeon_list, -1, -1, 2, -1);
    let map = vec![room_1, room_2, room_3, room_4];
    let actual_world = game::World::new(map);
    let possible_worlds = vec![];
    let game = game::Game::new(actual_world, possible_worlds, 0);
    game.show_intro();
    loop {
        let mut input = String::new();
        print!("> ");
        std::io::stdout().flush().unwrap();
        let _ = std::io::stdin().read_line(&mut input);
        let output = "test output".to_string();
        if input == "q\r\n" {
            println!("You quit the game");
            break;
        } else {
            println!("{}",output);
        }

    }
}
