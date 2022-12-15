#![feature(is_some_and)]

use std::ops::Add;
use std::collections::HashMap;

// =========================================== TYPE AND STRUCT DEFINITIONS ===========================================

type TreeSize = u32;

type CoordinateUnit = i32;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Coordinate {
    pub x: CoordinateUnit,
    pub y: CoordinateUnit
}

pub struct Tree {
    size: TreeSize,

    visible_north: Option<bool>,
    visible_south: Option<bool>,
    visible_east: Option<bool>,
    visible_west: Option<bool>,

    visible_trees_north: Option<usize>,
    visible_trees_south: Option<usize>,
    visible_trees_east: Option<usize>,
    visible_trees_west: Option<usize>,
}

pub struct Forest {
    min_pos: Coordinate,
    max_pos: Coordinate,
    pub trees: HashMap<Coordinate, Tree>
}

// =============================================== AUXILIARY FUNCTIONS ===============================================

fn check_visibility_direction(start_pos: Coordinate, iteration_delta: Coordinate, iteration_clear: Coordinate, forest: &mut Forest, mut callback: Box<dyn FnMut(&mut Tree, bool)>) {

    let mut current_line: Coordinate = start_pos;

    while current_line.x >= forest.min_pos.x && current_line.x <= forest.max_pos.x &&
        current_line.y >= forest.min_pos.y && current_line.y <= forest.max_pos.y {

            let mut current_pos: Coordinate = current_line;
            let mut current_max_size: Option<TreeSize> = None;

            while current_pos.x >= forest.min_pos.x && current_pos.x <= forest.max_pos.x &&
                current_pos.y >= forest.min_pos.y && current_pos.y <= forest.max_pos.y {

                    let current_tree: &mut Tree = forest.trees.get_mut(&current_pos).unwrap();
                    if current_max_size.is_none() || current_max_size.is_some_and(|max_size| max_size < current_tree.size) {

                        current_max_size = Some(current_tree.size);
                        callback(current_tree, true);

                    } else { callback(current_tree, false) }

                    current_pos = current_pos + iteration_delta;
            }

            current_line = current_line + iteration_clear;
    }
}

pub fn check_visibility_outside_forest(forest: &mut Forest) {
    check_visibility_direction(Coordinate::new(forest.min_pos.x, forest.min_pos.y),
        Coordinate::new(0, 1),
        Coordinate::new(1, 0),
        forest,
        Box::new(|tree, visibility| tree.set_visible_north(visibility)));
    check_visibility_direction(Coordinate::new(forest.min_pos.x, forest.max_pos.y),
        Coordinate::new(0, -1),
        Coordinate::new(1, 0),
        forest,
        Box::new(|tree, visibility| tree.set_visible_south(visibility)));
    check_visibility_direction(Coordinate::new(forest.max_pos.x, forest.min_pos.y),
        Coordinate::new(-1, 0),
        Coordinate::new(0, 1),
        forest,
        Box::new(|tree, visibility| tree.set_visible_east(visibility)));
    check_visibility_direction(Coordinate::new(forest.min_pos.x, forest.min_pos.y),
        Coordinate::new(1, 0),
        Coordinate::new(0, 1),
        forest,
        Box::new(|tree, visibility| tree.set_visible_west(visibility)));
}

fn check_visibility_position_direction(start_pos: Coordinate, iteration_delta: Coordinate, forest: &mut Forest, mut callback: Box<dyn FnMut(&mut Tree, usize)>) {

    let focus_tree_size: TreeSize = forest.trees.get_mut(&start_pos).unwrap().size;
    let mut current_pos: Coordinate = start_pos + iteration_delta;

    let mut count_visible_trees: usize = 0;
    let mut current_threshold: Option<TreeSize> = None;

    while current_pos.x >= forest.min_pos.x && current_pos.x <= forest.max_pos.x &&
        current_pos.y >= forest.min_pos.y && current_pos.y <= forest.max_pos.y &&
        (current_threshold.is_none() || current_threshold.is_some_and(|threshold| threshold < focus_tree_size)) {

            let current_tree: &Tree = forest.trees.get(&current_pos).unwrap();
            count_visible_trees = count_visible_trees + 1;
            current_threshold = Some(current_tree.size);

            current_pos = current_pos + iteration_delta;
    }

    let focus_tree: &mut Tree = forest.trees.get_mut(&start_pos).unwrap();
    callback(focus_tree, count_visible_trees);
}

fn check_visibility_position(forest: &mut Forest, position: Coordinate) {
    check_visibility_position_direction(position,
        Coordinate::new(0, -1),
        forest,
        Box::new(|tree, visibility| tree.set_visible_trees_north(visibility)));
    check_visibility_position_direction(position,
        Coordinate::new(0, 1),
        forest,
        Box::new(|tree, visibility| tree.set_visible_trees_south(visibility)));
    check_visibility_position_direction(position,
        Coordinate::new(1, 0),
        forest,
        Box::new(|tree, visibility| tree.set_visible_trees_east(visibility)));
    check_visibility_position_direction(position,
        Coordinate::new(-1, 0),
        forest,
        Box::new(|tree, visibility| tree.set_visible_trees_west(visibility)));
}

pub fn check_visibility_inside_forest(forest: &mut Forest) {

    for x in forest.min_pos.x ..= forest.max_pos.x {
        for y in forest.min_pos.y ..= forest.max_pos.y {

            let coordinate: Coordinate = Coordinate::new(x, y);
            check_visibility_position(forest, coordinate);
        }
    }
}

// ================================================= IMPLEMENTATIONS =================================================

impl Coordinate {

    fn new(x: CoordinateUnit, y: CoordinateUnit) -> Coordinate {
        Coordinate { x, y }
    }
}

impl Add for Coordinate {

    type Output = Self;
    
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Tree {

    fn new(size: TreeSize) -> Tree {
        Tree { size: size,
            visible_north: None, visible_south: None, visible_east: None, visible_west: None,
            visible_trees_north: None, visible_trees_south: None, visible_trees_east: None, visible_trees_west: None }
    }

    fn set_visible_north(&mut self, visibility: bool) { self.visible_north = Some(visibility) }
    fn set_visible_south(&mut self, visibility: bool) { self.visible_south = Some(visibility) }
    fn set_visible_east(&mut self, visibility: bool) { self.visible_east = Some(visibility) }
    fn set_visible_west(&mut self, visibility: bool) { self.visible_west = Some(visibility) }

    fn set_visible_trees_north(&mut self, count: usize) { self.visible_trees_north = Some(count) }
    fn set_visible_trees_south(&mut self, count: usize) { self.visible_trees_south = Some(count) }
    fn set_visible_trees_east(&mut self, count: usize) { self.visible_trees_east = Some(count) }
    fn set_visible_trees_west(&mut self, count: usize) { self.visible_trees_west = Some(count) }

    pub fn check_visible(&self) -> bool {
        return self.visible_north.is_some_and(|visible| visible) ||
            self.visible_south.is_some_and(|visible| visible) ||
            self.visible_east.is_some_and(|visible| visible) ||
            self.visible_west.is_some_and(|visible| visible);
    }

    pub fn scenic_score(&self) -> usize {
        return self.visible_trees_north.unwrap() * self.visible_trees_south.unwrap() *
            self.visible_trees_east.unwrap() * self.visible_trees_west.unwrap();
    }
}

impl Forest {

    pub fn new(trees_sizes: Vec<Vec<TreeSize>>) -> Forest {

        let mut trees: HashMap<Coordinate, Tree> = HashMap::new();
        let mut min_pos: Coordinate = Coordinate::new(0, 0);
        let mut max_pos: Coordinate = Coordinate::new(0, 0);

        for (y, row_tree_sizes) in trees_sizes.into_iter().enumerate() {
            for (x, tree_size) in row_tree_sizes.into_iter().enumerate() {

                let coordinate: Coordinate = Coordinate::new(x as CoordinateUnit, y as CoordinateUnit);
                let tree: Tree = Tree::new(tree_size);
                trees.insert(coordinate, tree);

                if coordinate.x < min_pos.x { min_pos.x = coordinate.x }
                if coordinate.x > max_pos.x { max_pos.x = coordinate.x }
                if coordinate.y < min_pos.y { min_pos.y = coordinate.y }
                if coordinate.y > max_pos.y { max_pos.y = coordinate.y }

            }
        }

        return Forest { min_pos, max_pos, trees }
    }
}