use super::domain::direction::Direction;
use super::domain::point::{Point, ONE_DOWN, ONE_LEFT};
use super::domain::shape::Shape;
use super::domain::shapes::{Shapes, SHAPE_ORDER};

const CHAMBER_WIDTH: i32 = 7;

pub fn start(input: &str) -> String {
    let mut shape_cycle = SHAPE_ORDER.iter().cycle();
    let mut dir_cycle = input.chars().cycle();
    let mut rested_shapes = vec![];
    'outer: while let Some(mut shape) = shape_cycle.next().map(|s| s.clone()) {
        println!("{:?}", rested_shapes);
        let spawn = get_spawn_point(&rested_shapes);
        shape.shift_to_pt(&spawn);
        while let Some(direction) = dir_cycle.next().map(|dir| Direction::from(&dir)) {
            // dont move the shape here, but rather test it in a seperate function
            match has_no_collision(direction, &shape, &rested_shapes) {
                true => shape.shift(direction),
                false => {
                    rested_shapes.push(shape);
                    continue 'outer;
                }
            }
            match has_no_collision(Direction::DOWN, &shape, &rested_shapes) {
                true => shape.shift(Direction::DOWN),
                false => {
                    rested_shapes.push(shape);
                    continue 'outer;
                }
            }
        }
        println!("{:?}", rested_shapes);
    }

    "haloe".into()
}

fn get_spawn_point(rested_shapes: &Vec<Shape>) -> Point {
    let hight = rested_shapes
        .iter()
        .map(|sh| sh.get_max_hight())
        .max()
        .unwrap_or_default();
    Point::new(3, hight + 3)
}

fn has_no_collision(direction: Direction, shape: &Shape, rested_shapes: &Vec<Shape>) -> bool {
    with_chamber(direction, shape) || with_other_shape(direction, shape, rested_shapes)
}

fn with_chamber(direction: Direction, shape: &Shape) -> bool {
    let mut test_shape = shape.clone();
    test_shape.shift(direction);
    test_shape.get_points().iter().all(|pt| {
        if pt.x > 0 || pt.x <= CHAMBER_WIDTH {
            return true;
        }
        return false;
    })
}
fn with_other_shape(direction: Direction, shape: &Shape, rested_shapes: &Vec<Shape>) -> bool {
    let mut test_shape = shape.clone();
    test_shape.shift(direction);
    for rest_shape in rested_shapes {
        if shape.colides(rest_shape) {
            return false;
        }
    }
    true
}
