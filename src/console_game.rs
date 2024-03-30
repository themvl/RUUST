use std::fmt::{Debug, Display};

use crate::game_state::{Clearing, MapType, Player, Token};
use itertools::join;

use super::game_state;
use dialoguer::Select;
use petgraph::graph::NodeIndex;

pub fn run() {
    let mut map = setup_board();
    setup_marquise(&mut map);
}

fn setup_board() -> game_state::Map {
    game_state::initialize_map(MapType::Fall)
}

fn setup_marquise(map: &mut game_state::Map) {
    let corners = map.get_corners();
    println!("The map:");
    print_clearings(map, map.get_clearings());
    let corners: Vec<usize> = corners.iter().map(|x| x.0.index()).collect();
    println!("choose corner to start marquise: ");
    let corner = Select::new()
        .with_prompt("What do you choose?")
        .items(&corners)
        .interact()
        .unwrap();

    println!("you chose: {}", corner);

    map.place_keep(NodeIndex::new(*corners.get(corner).unwrap()));
    print_clearings(map, map.get_clearings());
}

fn print_clearings<I>(map: &game_state::Map, clearings: I)
where
    I: IntoIterator<Item = NodeIndex<u8>>,
{
    let iter = clearings.into_iter();
    for clearing in iter {
        println!(
            "{} {} connected: {}",
            clearing.index(),
            map.get_clearing(clearing),
            join(map.get_connected(clearing).map(|x| x.index()), ",")
        );
    }
}

impl Display for game_state::Clearing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?} spots: {:?}{}{}{}",
            self.suit,
            self.build_spots,
            if self.tokens.get(&Token::Keep).unwrap_or(&0) == &0u8 {
                ""
            } else {
                " K"
            },
            if self.tokens.get(&Token::Wood).unwrap_or(&0) == &0u8 {
                "".to_string()
            } else {
                format!(" W: {}", self.tokens.get(&Token::Wood).unwrap())
            },
            if self.warriors.get(&Player::Marquise).unwrap_or(&0) == &0u8 {
                "".to_string()
            } else {
                format!(" WM:{}", self.warriors.get(&Player::Marquise).unwrap())
            }
        )
    }
}
