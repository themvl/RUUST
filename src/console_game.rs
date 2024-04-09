use std::fmt::Display;

use crate::game_state::{Faction, MapType, Suit, Token};
use itertools::{join, Itertools};

use super::game_state;
use console::Term;
use dialoguer::Select;
use petgraph::{csr::IndexType, graph::NodeIndex};
use std::fmt;

pub fn run() {
    let term = Term::stdout();
    let mut map = setup_board();
    setup_marquise(&mut map, &term);
    setup_eyrie(&mut map, &term);
}

fn setup_board() -> game_state::Map {
    game_state::initialize_map(MapType::Fall)
}

fn setup_marquise(map: &mut game_state::Map, term: &Term) {
    term.clear_screen();
    let corners = map.get_corners();

    print_clearings(map, map.get_clearings());
    let corners: Vec<usize> = corners.iter().map(|x| x.0.index()).collect();
    println!("choose corner to start marquise: ");
    let corner = Select::new()
        .with_prompt("What do you choose?")
        .items(&corners)
        .interact()
        .unwrap();

    println!("you chose: {}", corner);

    let keep = NodeIndex::new(*corners.get(corner).unwrap());
    map.place_keep(keep);

    term.clear_screen();
    print_clearings(map, map.get_clearings());

    let options: Vec<usize> = map
        .get_marquise_start_building_options()
        .map(|x| x.index())
        .collect();
    ask_to_build(
        map,
        "Choose where to place sawmill",
        game_state::Structure::Sawmill,
        options,
    );

    term.clear_screen();
    print_clearings(map, map.get_clearings());

    let options: Vec<usize> = map
        .get_marquise_start_building_options()
        .map(|x| x.index())
        .collect();
    ask_to_build(
        map,
        "Choose where to place Workshop",
        game_state::Structure::Workshop,
        options,
    );

    term.clear_screen();
    print_clearings(map, map.get_clearings());

    let options: Vec<usize> = map
        .get_marquise_start_building_options()
        .map(|x| x.index())
        .collect_vec();
    ask_to_build(
        map,
        "Choose where to place Recruiter",
        game_state::Structure::Recruiter,
        options,
    );

    term.clear_screen();
    print_clearings(map, map.get_clearings());
}

fn setup_eyrie(map: &mut game_state::Map, term: &Term) {
    term.clear_screen();
    print_clearings(map, map.get_clearings());

    let first_roost = ask_to_build(
        map,
        "Where to place first roost?",
        game_state::Structure::Roost,
        map.get_eyrie_start_options()
            .map(|x| x.index())
            .collect_vec(),
    );

    map.place_warrior(first_roost, game_state::Faction::Eyrie, 6);

    term.clear_screen();
    print_clearings(map, map.get_clearings());
}

fn ask_to_build(
    map: &mut game_state::Map,
    message: &str,
    structure: game_state::Structure,
    options: Vec<usize>,
) -> NodeIndex<u8> {
    println!("{}", message);
    let clearing = Select::new()
        .with_prompt("What do you choose?")
        .items(&options)
        .interact()
        .unwrap();
    let index = NodeIndex::new(*options.get(clearing).unwrap());
    map.place_structure(index, structure);
    index
}

fn print_clearings<I>(map: &game_state::Map, clearings: I)
where
    I: IntoIterator<Item = NodeIndex<u8>>,
{
    println!("The map:");
    let iter = clearings.into_iter();
    for clearing in iter {
        println!(
            "{:2} {:-<15} connected: {}",
            clearing.index(),
            map.get_clearing(clearing),
            join(map.get_connected(clearing).map(|x| x.index()), ",")
        );
    }
}

impl Display for game_state::Clearing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut building_string = String::new();
        for building in &self.structures {
            building_string.push_str(match building {
                game_state::Structure::Recruiter => "Re",
                game_state::Structure::Roost => "Ro",
                game_state::Structure::Sawmill => "S",
                game_state::Structure::Workshop => "W",
            });
        }

        write!(
            f,
            "{:6} spots:{:2}({}) {:1} B:{:5}{:3} Wa {:4}{:4}",
            self.suit,
            self.build_spots,
            self.structures.len(),
            if self.tokens.get(&Token::Keep).unwrap_or(&0) == &0u8 {
                ""
            } else {
                "K"
            },
            building_string,
            if self.tokens.get(&Token::Wood).unwrap_or(&0) == &0u8 {
                "".to_string()
            } else {
                format!("W:{}", self.tokens.get(&Token::Wood).unwrap())
            },
            if self.warriors.get(&Faction::Marquise).unwrap_or(&0) == &0u8 {
                "".to_string()
            } else {
                format!("M:{}", self.warriors.get(&Faction::Marquise).unwrap())
            },
            if self.warriors.get(&Faction::Eyrie).unwrap_or(&0) == &0u8 {
                "".to_string()
            } else {
                format!("E:{}", self.warriors.get(&Faction::Eyrie).unwrap())
            }
        )
    }
}

impl Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string = match self {
            Suit::Bird => "Bird",
            Suit::Rabbit => "Rabbit",
            Suit::Fox => "Fox",
            Suit::Mouse => "Mouse",
            Suit::Wolf => "Wolf",
        };

        string.fmt(f).and_then(|_| write!(f, ""))
    }
}
