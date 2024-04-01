use core::fmt;
use std::{collections::HashMap, iter, path::Display};

use itertools::{concat, Itertools};
use petgraph::{
    data::DataMap,
    graph::{Neighbors, NodeIndex},
    visit::{EdgeIndexable, IntoNodeIdentifiers, NodeIndexable},
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
pub enum Faction {
    Marquise,
    Eyrie,
    None,
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum Token {
    Wood,
    Keep,
}

pub enum Structure {
    Recruiter,
    Sawmill,
    Workshop,
    Roost,
}

pub struct Clearing {
    pub suit: Suit,
    pub structures: Vec<Structure>,
    pub build_spots: u8,
    pub warriors: HashMap<Faction, u8>,
    pub tokens: HashMap<Token, u8>,
    corner: bool,
    rule: Faction,
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
    keep: Option<NodeIndex<u8>>,
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
        // check if index is corner
        if !self.corners.contains_key(&index) {
            panic!("illigal move!")
        }

        // place keep token
        self.graph
            .node_weight_mut(index)
            .unwrap()
            .tokens
            .insert(Token::Keep, 1);

        self.keep = Some(index);

        // find opposite corner and get every clearing except this one
        let opposite = self.corners.get(&index).unwrap().to_owned();
        for clearing in self.graph.node_identifiers() {
            if clearing != opposite {
                self.place_warrior(clearing, Faction::Marquise, 1);
            }
        }
    }

    pub fn get_clearings(&self) -> Vec<NodeIndex<u8>> {
        self.graph.node_indices().collect_vec()
    }

    pub fn place_warrior(&mut self, index: NodeIndex<u8>, warrior: Faction, amount: u8) {
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

    pub fn place_structure(&mut self, index: NodeIndex<u8>, structure: Structure) {
        let clearing = self.graph.node_weight_mut(index).unwrap();
        if clearing.buildable() {
            clearing.structures.push(structure);
        }
    }

    pub fn setup_eyrie(&mut self, corner: NodeIndex<u8>) {
        // check if corner is allowed
        if !self.corners.contains_key(&corner) {
            panic!("not a corner!")
        }
        if self
            .graph
            .node_weight(corner)
            .unwrap()
            .tokens
            .contains_key(&Token::Keep)
        {
            panic!("corner with keep already placed")
        }

        // place a roost
        self.place_structure(corner, Structure::Roost);

        // place 6 warriors
        self.place_warrior(corner, Faction::Eyrie, 6);
    }

    pub fn get_marquise_start_building_options(&self) -> impl Iterator<Item = NodeIndex<u8>> + '_ {
        self.get_connected(self.keep.unwrap())
            .chain(iter::once(self.keep.unwrap()))
            .filter(|x| self.get_clearing(*x).buildable())
    }

    pub fn get_eyrie_start_options(&self) -> impl Iterator<Item = &NodeIndex<u8>> + '_ {
        self.get_corners().into_iter().filter_map(|x| {
            if !self.get_clearing(*x.0).tokens.contains_key(&Token::Keep) {
                Some(x.0)
            } else {
                None
            }
        })
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
            rule: Faction::None,
        }
    }
}

impl Default for Map {
    fn default() -> Self {
        Map {
            graph: Graph::<Clearing, (), Undirected, u8>::default(),
            corners: HashMap::new(),
            keep_placed: false,
            keep: None,
        }
    }
}

impl Clearing {
    pub fn buildable(&self) -> bool {
        usize::from(self.build_spots) > self.structures.len()
    }
}
