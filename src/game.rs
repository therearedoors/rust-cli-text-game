use crate::globals::Direction;
use crate::things::{room::Room, actor::Actor, thing::{Thing, ThingHolder}};

pub struct Game {
    actual_world: World,
    possible_worlds: Vec<World>,
    player: Actor,
    commands: Vec<String>
}

impl Game {
    pub fn new(aw: World, pw: Vec<World>, player_start: isize) -> Game {
        Game {
            actual_world: aw,
            possible_worlds: pw,
            player: Actor::new("player".to_owned(), "a lovable game-player".to_owned(), player_start),
            commands: vec!["take".to_owned(), "drop".to_owned(), "look".to_owned(), "n".to_owned(), "s".to_owned(), "e".to_owned(), "w".to_owned()]}
    }

    pub fn get_player_ref(&self) -> &Actor {
        &self.player
    }

    pub fn get_player(&mut self) -> &mut Actor {
        &mut self.player
    }

    pub fn show_intro(&self) {
        println!("You find yourself in an interplanar spacecraft. This is the actual world. What do you want to do? [Enter n, s, w or e to travel, or q to quit the game.]");
    }

    pub fn run_command(game: &mut Game, command: &str) -> String {
        let command_list: Vec<String>;
        let mut output = String::from("ok");
        let clean_command = command.trim().to_lowercase();
        if clean_command != "q" {
            if clean_command == "" {
                output = String::from("You must enter a command.");
            } else {
            command_list = Self::word_list(&clean_command);
            output = Self::parse_command(game, &command_list);
            }
        }
        output
    }

    fn parse_command(game: &mut Game, command: &Vec<String>) -> String {
        let output;
        if command.len() == 1 {
            output = Self::process_verb(game, command);
        } else if command.len() == 2 {
            output = Self::process_verb_noun(game, command);
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
                "n" => {msg = Self::go_in_direction(game, room, Direction::North);},
                "s" => {msg = Self::go_in_direction(game, room, Direction::South);},
                "e" => {msg = Self::go_in_direction(game, room, Direction::East);},
                "w" => {msg = Self::go_in_direction(game, room, Direction::West);},
                // "take" => msg = String::from("You take the {}.", noun),
                // "drop" => msg = String::from("You drop the {}.", noun),
                "look" => {msg = Self::look(game);},
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

    fn go_in_direction(game: &mut Game, room: isize, direction: Direction) -> String {
        let room_number = game.player.move_to(&game.actual_world.map, room);
        Self::update_output(game, room_number)
    }

    fn look(game: &mut Game) -> String {
        if let Some(location) = game.actual_world.map.get(game.player.get_location() as usize) {
            return "You are in the ".to_owned() + location.describe().clone().as_str();
        }
        "Nothing to be seen".to_owned()
    }
    
    fn update_output(&self, room_number: i32) -> String {
        let s: String;
        if room_number == Direction::NOEXIT as i32 {
            s = String::from("No Exit!");
        } else {
            let rooms =  &self.actual_world.map;
            let &idx = &self.get_player_ref().get_location();
            let r = rooms.get(idx as usize).unwrap();
            let list_string = r.get_things()
                .into_iter()
                .map(|t| format!("{}: {}", t.get_name(), t.get_description()))
                .collect::<Vec<String>>()
                .join("\n");
            s = format!("You are in {}. {}\nThings here:\n{}", r.get_name(), r.get_description(), list_string);
        }
        s
    } 
}

pub struct World {
    map: Vec<Room>
}

impl World {
    pub fn new(m: Vec<Room>) -> World {
        World {
            map: m
        }
    }
}

pub enum Shape {
    Square,
    Triangle,
    Circle,
    Retangle,
    Custom(Vec<Shape>)
}

pub enum Color{
    Red(u8),
    Green(u8),
    Blue(u8),
}