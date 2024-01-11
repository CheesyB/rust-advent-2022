use super::domain::direction::{self, Direction};
use super::domain::point::{Point, ONE_DOWN, ONE_LEFT};
use super::domain::shape::Shape;
use super::domain::shapes::{Shapes, SHAPE_ORDER};

const CHAMBER_WIDTH: i32 = 7;

struct ShapeSpawner;

pub fn start(input: &str) -> String {
    let mut shape_cycle = SHAPE_ORDER.iter().cycle();
    let mut dir_cycle = input.chars().cycle();
    let mut rested_shapes = vec![];
    while let Some(mut shape) = shape_cycle.next().map(|s| s.clone()) {
        let spawn = get_spawn_point(rested_shapes);
        shape.shift(&spawn);
        while let Some(direction) = dir_cycle.next().map(|dir| Direction::from(&dir).value()) {
            // dont move the shape here, but rather test it in a seperate function
            shape.shift(direction);
            // handle collision
            shape.shift(ONE_DOWN)
            //handle collision
        }
    }

    "haloe".into()
}

fn get_spawn_point(rested_shapes: Vec<Shape>) -> Point {
    //todo get hight
    let hight = rested_shapes.iter().max().unwrap().y;
    Point::new(3, hight + 3)
}

fn has_collision(shape: &Shape, rested_shapes: Vec<Shape>) -> bool {
    let test_shape = shape.clone();

    with_chamber() || with_other_shape()
}

fn with_chamber() -> bool {
    false
}
fn with_other_shape() -> bool {
    false
}
