use std::{ops::Add, collections::HashSet};

// =========================================== TYPE AND STRUCT DEFINITIONS ===========================================

type CoordinateUnit = i32;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Coordinate {
    x: CoordinateUnit,
    y: CoordinateUnit
}

#[derive(Clone, Copy)]
enum MovementDirection {
    Up,
    Down,
    Right,
    Left
}

pub struct Rope {
    knots: Vec<Vec<Coordinate>>
}

// =============================================== AUXILIARY FUNCTIONS ===============================================

fn convert_set_of_movements(movement_lines: &Vec<String>) -> Vec<MovementDirection> {

    let mut movements: Vec<MovementDirection> = Vec::new();
    for line in movement_lines.into_iter() {

        let mut line_split: Vec<String> = line.split_whitespace()
            .map(|split| split.to_owned())
            .collect();

        let movement_char: char = line_split.remove(0)
            .chars()
            .collect::<Vec<char>>()
            .remove(0);
        let movement_direction: MovementDirection = MovementDirection::get_movement_from_char(movement_char);

        let movement_units: u32 = line_split.remove(0).parse().unwrap();

        for _ in 0 .. movement_units {
            movements.push(movement_direction.clone());
        }
    }

    return movements;
}

fn get_knot_movement(new_head_knot: &Coordinate, current_tail_knot: &Coordinate) -> Coordinate {

    let distance_tail_to_new_head: CoordinateUnit = new_head_knot.distance(current_tail_knot);
    let new_tail: Coordinate;

    if distance_tail_to_new_head > 2 {
        
        let mut current_best: Option<(Coordinate, CoordinateUnit)> = None;
        for delta_to_test in vec![Coordinate::new(1, 0), Coordinate::new(-1, 0), Coordinate::new(0, 1), Coordinate::new(0, -1),
            Coordinate::new(1, 1), Coordinate::new(1, -1), Coordinate::new(-1, -1), Coordinate::new(-1, 1)].into_iter() {

                let new_position = current_tail_knot.clone() + delta_to_test;
                let distance: CoordinateUnit = new_head_knot.distance(&new_position);
                if current_best.is_none() || current_best.unwrap().1 > distance {
                    current_best = Some((new_position, distance));
                }
            }

        new_tail = current_best.unwrap().0;

    } else { new_tail = current_tail_knot.clone() }

    return new_tail;
}

// ================================================= IMPLEMENTATIONS =================================================

impl Coordinate {

    fn new(x: CoordinateUnit, y: CoordinateUnit) -> Coordinate {
        Coordinate { x, y }
    }

    fn distance(&self, other: &Coordinate) -> CoordinateUnit {
        
        let delta_x: CoordinateUnit = other.x - self.x;
        let delta_y: CoordinateUnit = other.y - self.y;

        return delta_x.pow(2) + delta_y.pow(2);
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

impl MovementDirection {

    fn get_delta_direction(&self) -> Coordinate {
        match self {
            &MovementDirection::Up      => Coordinate::new( 0,  1),
            &MovementDirection::Down    => Coordinate::new( 0, -1),
            &MovementDirection::Right   => Coordinate::new( 1,  0),
            &MovementDirection::Left    => Coordinate::new(-1,  0),
        }
    }

    fn get_movement_from_char(code: char) -> MovementDirection {
        match code {
            'U' => MovementDirection::Up,
            'D' => MovementDirection::Down,
            'R' => MovementDirection::Right,
            'L' => MovementDirection::Left,
             _  => panic!("🚨 Movement code '{}' not recognized", code)
        }
    }
}

impl Rope {

    pub fn new(movement_lines: &Vec<String>, number_knots: usize) -> Rope {

        let movements: Vec<MovementDirection> = convert_set_of_movements(movement_lines);
        let mut rope: Rope = Rope {
            knots: (0 .. number_knots).into_iter()
                .map(|_| vec![Coordinate::new(0, 0)])
                .collect()
        };

        for movement in movements { rope.make_iteration(movement) }
        return rope;
    }

    fn make_iteration(&mut self, movement_head: MovementDirection) {

        let current_head: &Coordinate = self.knots.first().unwrap()
            .last().unwrap();
        
        let movement_head_delta: Coordinate = movement_head.get_delta_direction();
        let mut new_head: Coordinate = current_head.clone() + movement_head_delta;

        self.knots.first_mut().unwrap().push(new_head.clone());

        for knot_positions in self.knots.iter_mut().skip(1) {

            let current_tail: &Coordinate = knot_positions.last().unwrap();
            let new_tail: Coordinate = get_knot_movement(&new_head, current_tail);

            knot_positions.push(new_tail.clone());
            new_head = new_tail.clone();
        }
    }

    pub fn get_tail_number_different_positions(&self) -> usize {
        self.knots.last().unwrap()
            .iter()
            .map(|&coordinate| coordinate.clone())
            .collect::<HashSet<Coordinate>>()
            .len()
    }
}