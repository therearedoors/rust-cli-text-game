use std::io::Write;
use crate::things::{room::Room, treasure::Treasure, info::Info};

mod game;
mod globals;
mod things;
mod traits;

fn main() {
    let treasure_ring = Treasure::new(Info {name: "ring".to_owned(), description: "It is a ring of great power".to_owned()}, 500);
    let treasure_wombat = Treasure::new(Info {name: "wombat".to_owned(), description: "It is a cuddly little wombat. It is squeaking gently to itself.".to_owned()}, 700);
    let dungeon_list: Vec<Treasure> = vec![treasure_ring, treasure_wombat];
    let room_1 = Room::new(Info {name:"Troll Room".to_owned(), description: "A dank room that smells of troll".to_owned()}, Box::new(vec![]), -1, 2, -1, 1);
    let room_2 = Room::new(Info {name:"Forest".to_owned(), description: "A leafy woodland".to_owned()}, Box::new(vec![]), -1, -1, 0, -1);
    let room_3 = Room::new(Info {name:"Cave".to_owned(), description: "A dismal cave with walls covered in luminous moss".to_owned()}, Box::new(vec![]), 0, -1, -1, 3);
    let room_4 = Room::new(Info {name:"Dungeon".to_owned(), description: "A nasty, dark cell".to_owned()}, Box::new(dungeon_list), -1, -1, 2, -1);
    let map = vec![room_1, room_2, room_3, room_4];
    let actual_world = game::World::new(map);
    let possible_worlds = vec![];
    let mut game = game::Game::new(actual_world, possible_worlds, 0);
    game.show_intro();
    loop {
        let mut input = String::new();
        print!("> ");
        std::io::stdout().flush().unwrap();
        let _ = std::io::stdin().read_line(&mut input);
        let output = game::Game::run_command(&mut game, &input);
        if input == "q\r\n" {
            println!("You quit the game");
            break;
        } else {
            println!("{}",output);
        }

    }
}
