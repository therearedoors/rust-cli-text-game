use crate::globals::{Direction};
use crate::things::{room::Room, actor::Actor, thing::{ThingHolder,Thingsein}};

pub struct Game {
    map: Vec<Room>,
    player: Actor,
    commands: Vec<String>,
}

impl Game {
    pub fn new(map: Vec<Room>, player_start: Room) -> Game {
        Game {
            map,
            player: Actor::new("player".to_owned(), "a lovable game-player".to_owned(), player_start),
            commands: vec!["take".to_owned(), "drop".to_owned(), "look".to_owned(), "n".to_owned(), "s".to_owned(), "e".to_owned(), "w".to_owned()]}
    }

    pub fn get_player(&mut self) -> &mut Actor {
        &mut self.player
    }

    // fn move_player_to(&mut self, dir: Direction) -> i32 {
    //     let player = self.get_player();
    //     let room = player.get_location();
    //     player.move_to(self.map.clone(), room, dir)
    // }
    

}

pub fn show_intro() {
    println!("\nYou have fallen down a rabbit hole and arrived in
an underground cavern that smells strongly of troll.
Where do you want to go? [Enter n, s, w or e]?
(or enter q to quit)");
}

pub fn run_command(game: &mut Game, command: &str) -> String {
    let command_list: Vec<String>;
    let mut output = String::from("ok");
    let clean_command = command.trim().to_lowercase();
    if clean_command != "q" {
        if clean_command == "" {
            output = String::from("You must enter a command.");
        } else {
        command_list = word_list(&clean_command);
        output = parse_command(game, &command_list);
        }
    }
    output
}

fn parse_command(game: &mut Game, command: &Vec<String>) -> String {
    let output;
    if command.len() == 1 {
        output = process_verb(game, command);
    } else if command.len() == 2 {
        output = process_verb_noun(game, command);
    } else {
        output = String::from("Only two word commands allowed!");
    }
    output
}

fn word_list(command: &String) -> Vec<String> {
    let delims = "[ \t,.:;?!\"']+";
    command.split(delims).map(String::from).collect::<Vec<String>>()
}

fn process_verb(game: &mut Game, wordlist: &Vec<String>) -> String {
    let verb_op = wordlist.get(0);
    println!("{:?}", wordlist);
    let mut msg = String::new();
    let room = game.player.get_location();
    match verb_op {
        Some(verb) => match verb.as_str() {
            "n" => {msg = go_n(game, room);},
            "s" => {msg = go_s(game, room);},
            "e" => {msg = go_e(game, room);},
            "w" => {msg = go_w(game, room);},
            // "take" => msg = String::from("You take the {}.", noun),
            // "drop" => msg = String::from("You drop the {}.", noun),
            "look" => {msg = look(game);},
            _ => msg.push_str("not yet implemented"),
        },
        None => panic!("No verb entered"),
    };
    msg
}

fn process_verb_noun(game: &Game, wordlist: &Vec<String>) -> String {
    let verb = wordlist.get(0).unwrap();
    let noun = wordlist.get(1).unwrap();
    let mut msg = String::new();
    if !game.commands.contains(&verb) {
        msg = String::from(format!("{} {}",verb," is not a known verb!\n"));
    }
    if !game.commands.contains(&noun) {
        msg.push_str(noun);
        msg.push_str(" is not a known noun!\n");
    }
    msg
}

fn go_n(game: &mut Game, room: Room) -> String {
    let Game { map, player, ..} = game;
    let room_number = player.move_to(map.to_vec(), room, Direction::North);
    update_output(game, room_number)
}

fn go_s(game: &mut Game, room: Room) -> String {
    let Game { map, player, ..} = game;
    let room_number = player.move_to(map.to_vec(), room, Direction::South);
    update_output(game, room_number)
}

fn go_w(game: &mut Game, room: Room) -> String {
    let Game { map, player, ..} = game;
    let room_number = player.move_to(map.to_vec(), room, Direction::West);
    update_output(game, room_number)
}

fn go_e(game: &mut Game, room: Room) -> String {
    let Game { map, player, ..} = game;
    let room_number = player.move_to(map.to_vec(), room, Direction::East);
    update_output(game, room_number)
}

fn look(game: &mut Game) -> String {w
    "You are in the ".to_owned() + &game.get_player().get_location().describe()
}

fn update_output(game: &mut Game, room_number: i32) -> String {
    let s: String;
    if room_number == Direction::NOEXIT as i32 {
        s = String::from("No Exit!");
    } else {
        let player = game.get_player();
        let r = player.get_location();
        let list_string = r.get_things()
            .into_iter()
            .map(|t| format!("{}: {}", t.get_name(), t.get_description()))
            .collect::<Vec<String>>()
            .join("\n");
        s = format!("You are in {}. {}\nThings here:\n{}", r.thing.get_name(), r.thing.get_description(), list_string);
    }
    s
}