use std::collections::HashMap;

use itertools::Itertools;
use petgraph::{
    graph::{Neighbors, NodeIndex},
    visit::{EdgeIndexable, IntoNodeIdentifiers},
    Graph, Undirected,
};

#[derive(Debug)]
pub enum Suit {
    Bird,
    Wolf,
    Rabbit,
    Mouse,
    Fox,
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum Player {
    Marquise,
    Eyrie,
    None,
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum Token {
    Wood,
    Keep,
}

enum Structure {
    Recruiter,
    Sawmill,
    Workshop,
}

pub struct Clearing {
    pub suit: Suit,
    structures: Vec<Structure>,
    pub build_spots: u8,
    pub warriors: HashMap<Player, u8>,
    pub tokens: HashMap<Token, u8>,
    corner: bool,
    rule: Player,
}

pub enum MapType {
    Simple,
    V,
    Fall,
}

/// To initialize a map you need to initialize the graph with clearings
/// then define the corners
///

pub struct Map {
    graph: Graph<Clearing, (), Undirected, u8>,
    corners: HashMap<NodeIndex<u8>, NodeIndex<u8>>,
    keep_placed: bool,
}

pub struct Corner {
    pub corner: NodeIndex<u8>,
    pub opposite: NodeIndex<u8>,
}

pub fn initialize_map(map_type: MapType) -> Map {
    let graph = Graph::<Clearing, (), Undirected, u8>::default();
    let mut map = Map {
        graph,
        ..Default::default()
    };
    // change depending on map type

    match map_type {
        MapType::Simple => {
            let left = map.add_clearing(Suit::Bird, 2, true);
            let right = map.add_clearing(Suit::Bird, 1, true);
            map.extend_with_paths(&[(left, right)]);
            map.add_corner(left, right);
        }
        MapType::V => {
            let left = map.add_clearing(Suit::Bird, 2, true);
            let right = map.add_clearing(Suit::Bird, 1, true);
            let middle = map.add_clearing(Suit::Mouse, 1, false);
            map.extend_with_paths(&[(left, middle), (middle, right)]);
            map.add_corner(left, right);
        }
        MapType::Fall => {
            let topfox = map.add_clearing(Suit::Fox, 1, true);
            let toprabbit = map.add_clearing(Suit::Rabbit, 2, false);
            let topmouse = map.add_clearing(Suit::Mouse, 2, true);

            let leftmouse = map.add_clearing(Suit::Mouse, 2, false);
            let middlerabbit = map.add_clearing(Suit::Rabbit, 2, false);
            let middlefox = map.add_clearing(Suit::Fox, 2, false);

            let middlemouse = map.add_clearing(Suit::Mouse, 3, false);
            let rightfox = map.add_clearing(Suit::Fox, 2, false);

            let bottomrabbit = map.add_clearing(Suit::Rabbit, 1, true);
            let bottomfox = map.add_clearing(Suit::Fox, 2, false);
            let bottommouse = map.add_clearing(Suit::Mouse, 2, false);
            let bottomright = map.add_clearing(Suit::Rabbit, 1, true);

            map.extend_with_paths(&[
                (topfox, toprabbit),
                (toprabbit, topmouse),
                (topfox, middlerabbit),
                (topmouse, middlerabbit),
                (topfox, leftmouse),
                (middlerabbit, middlefox),
                (topmouse, rightfox),
                (leftmouse, middlefox),
                (middlefox, middlemouse),
                (middlemouse, rightfox),
                (leftmouse, bottomrabbit),
                (middlefox, bottomrabbit),
                (middlefox, bottommouse),
                (middlemouse, bottomright),
                (rightfox, bottomright),
                (bottomrabbit, bottomfox),
                (bottomfox, bottommouse),
                (bottommouse, bottomright),
            ]);

            map.add_corner(topfox, bottomright);
            map.add_corner(bottomrabbit, topmouse);
        }
    }

    map
}

impl Map {
    fn add_clearing(&mut self, suit: Suit, build_spots: u8, corner: bool) -> NodeIndex<u8> {
        self.graph.add_node(Clearing {
            suit,
            build_spots,
            structures: Vec::with_capacity(usize::from(build_spots)),
            corner,
            ..Default::default()
        })
    }
    // pub fn add_path() {}

    fn extend_with_paths<I>(&mut self, iterable: I)
    where
        I: IntoIterator,
        I::Item: petgraph::IntoWeightedEdge<()>,
        <I::Item as petgraph::IntoWeightedEdge<()>>::NodeId: Into<NodeIndex<u8>>,
    {
        self.graph.extend_with_edges(iterable);
    }

    pub fn get_corners(&self) -> &HashMap<NodeIndex<u8>, NodeIndex<u8>> {
        &self.corners
    }

    pub fn get_clearing(&self, index: NodeIndex<u8>) -> &Clearing {
        self.graph.node_weight(index).unwrap()
    }

    pub fn get_connected(&self, index: NodeIndex<u8>) -> Neighbors<'_, (), u8> {
        self.graph.neighbors_undirected(index)
    }

    pub fn place_keep(&mut self, index: NodeIndex<u8>) {
        // place keep token
        self.graph
            .node_weight_mut(index)
            .unwrap()
            .tokens
            .insert(Token::Keep, 1);

        // find opposite corner and get every clearing except this one
        let opposite = self.corners.get(&index).unwrap().to_owned();
        for clearing in self.graph.node_identifiers() {
            if clearing != opposite {
                self.place_warrior(clearing, Player::Marquise, 1);
            }
        }
    }

    pub fn get_clearings(&self) -> Vec<NodeIndex<u8>> {
        self.graph.node_indices().collect_vec()
    }

    pub fn place_warrior(&mut self, index: NodeIndex<u8>, warrior: Player, amount: u8) {
        self.graph
            .node_weight_mut(index)
            .unwrap()
            .warriors
            .entry(warrior)
            .and_modify(|x| *x += amount)
            .or_insert(amount);
    }

    fn add_corner(&mut self, corner: NodeIndex<u8>, opposite: NodeIndex<u8>) {
        self.corners.insert(corner, opposite);
        self.corners.insert(opposite, corner);
    }

    fn place_structure(&mut self, index: NodeIndex<u8>, structure: Structure) {
        let clearing = self.graph.node_weight_mut(index).unwrap();
        if usize::from(clearing.build_spots) < clearing.structures.len() {
            clearing.structures.push(structure);
        }
    }
}

impl Default for Clearing {
    fn default() -> Self {
        Clearing {
            suit: Suit::Bird,
            structures: Vec::new(),
            build_spots: 0,
            warriors: HashMap::with_capacity(4),
            tokens: HashMap::with_capacity(4),
            corner: false,
            rule: Player::None,
        }
    }
}

impl Default for Map {
    fn default() -> Self {
        Map {
            graph: Graph::<Clearing, (), Undirected, u8>::default(),
            corners: HashMap::new(),
            keep_placed: false,
        }
    }
}
