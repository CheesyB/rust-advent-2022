use itertools::Itertools;

use super::domain::direction::Direction;
use super::domain::point::Point;
use super::domain::shape::Shape;
use super::domain::shapes::SHAPE_ORDER;

const CHAMBER_WIDTH: i32 = 7;

pub fn simulate2(input: &str, iterrations: usize) -> i64 {
    let mut shape_cycle = SHAPE_ORDER.iter().cycle().enumerate().take(iterrations);
    let mut dir_cycle = input.chars().cycle();
    let mut rested_shapes: Vec<Shape> = vec![];
    let mut seen: Vec<(Vec<Point>, Direction, (i32, i32))> = vec![];
    let mut again = true;
    'outer: while let Some((count, mut shape)) = shape_cycle.next().map(|s| (s.0, s.1.clone())) {
        let spawn = get_spawn_point(&rested_shapes);
        shape.shift_to_pt(&spawn);
        if rested_shapes.len() > 10 {
            rested_shapes.drain(0..1);
        }
        while let Some(direction) = dir_cycle.next().map(|dir| Direction::from(&dir)) {
            let current_skyline = skyline(&rested_shapes, direction);
            for (ccount, ssk) in seen.iter().enumerate().skip(10091) {
                if ssk
                    .0
                    .iter()
                    .zip(current_skyline.0.iter())
                    .all(|(p1, p2)| p1 == p2)
                    && ssk.1 == current_skyline.1
                    && again
                {
                    again = false;

                    println!(" {} found at {} ", count, ccount);

                    break;
                }
            }

            match has_no_collision(direction, &shape, &rested_shapes) {
                true => {
                    shape.shift(direction);
                }
                false => {
                    seen.push(skyline(&rested_shapes, direction));
                }
            }
            match has_no_collision(Direction::DOWN, &shape, &rested_shapes) {
                true => {
                    shape.shift(Direction::DOWN);
                }
                false => {
                    rested_shapes.push(shape);
                    seen.push(skyline(&rested_shapes, direction));
                    continue 'outer;
                }
            }
        }
    }
    rested_shapes.last().unwrap().get_max_hight() as i64
}

fn skyline(
    rested_shapes: &Vec<Shape>,
    direction: Direction,
) -> (Vec<Point>, Direction, (i32, i32)) {
    let high = rested_shapes
        .iter()
        .map(|sh| sh.get_max_hight())
        .max()
        .unwrap_or_default();
    let low = rested_shapes
        .iter()
        .map(|sh| sh.ref_pt.y)
        .min()
        .unwrap_or_default();
    let significant = rested_shapes
        .iter()
        .rev()
        .take(5)
        .map(|sh| sh.ref_pt_relative(low))
        .collect_vec();
    (significant, direction, (high, low))
}

pub fn simulate(input: &str, iterrations: usize) -> i64 {
    let mut shape_cycle = SHAPE_ORDER.iter().cycle().enumerate().take(iterrations);
    let mut dir_cycle = input.chars().cycle();
    let mut rested_shapes: Vec<Shape> = vec![];
    'outer: while let Some((count, mut shape)) = shape_cycle.next().map(|s| (s.0, s.1.clone())) {
        let spawn = get_spawn_point(&rested_shapes);
        shape.shift_to_pt(&spawn);
        println!("{} {:}", count, shape);
        if rested_shapes.len() > 30 {
            rested_shapes.drain(0..10);
        }
        while let Some(direction) = dir_cycle.next().map(|dir| Direction::from(&dir)) {
            match has_no_collision(direction, &shape, &rested_shapes) {
                true => {
                    shape.shift(direction);
                    print!("{:?} ", direction);
                }
                false => (),
            }
            match has_no_collision(Direction::DOWN, &shape, &rested_shapes) {
                true => {
                    shape.shift(Direction::DOWN);
                    print!("{:?} ", Direction::DOWN);
                }
                false => {
                    rested_shapes.push(shape);
                    println!();
                    continue 'outer;
                }
            }
        }
    }
    rested_shapes.last().unwrap().get_max_hight() as i64
}

fn print_crack(rested_shapes: &Vec<Shape>, shape: &Shape) {
    let mut nrs = rested_shapes.clone();
    nrs.push(shape.clone());
    println!();
    let mut crack = vec![vec!['.'; 9]; 30];
    for rs in nrs {
        for pt in rs.get_points() {
            crack[pt.y as usize][pt.x as usize] = '#'
        }
    }
    for row in crack.iter().rev() {
        for col in row.iter().enumerate() {
            if col.0 == 0 || col.0 == 8 {
                print!("|");
            } else {
                print!("{} ", col.1);
            }
        }
        println!();
    }
}

fn get_spawn_point(rested_shapes: &Vec<Shape>) -> Point {
    let hight = rested_shapes
        .iter()
        .map(|sh| sh.get_max_hight())
        .max()
        .unwrap_or_default();
    Point::new(3, hight + 4)
}

fn has_no_collision(direction: Direction, shape: &Shape, rested_shapes: &Vec<Shape>) -> bool {
    with_chamber(direction, shape) && with_other_shape(direction, shape, rested_shapes)
}

fn with_chamber(direction: Direction, shape: &Shape) -> bool {
    let mut test_shape = shape.clone();
    test_shape.shift(direction);
    let wall = test_shape.get_points().iter().all(|pt| {
        if pt.x > 0 && pt.x <= CHAMBER_WIDTH {
            return true;
        }
        return false;
    });
    let bottom = test_shape.ref_pt.y != 0;
    return wall && bottom;
}
fn with_other_shape(direction: Direction, shape: &Shape, rested_shapes: &Vec<Shape>) -> bool {
    let mut test_shape = shape.clone();
    test_shape.shift(direction);
    for rest_shape in rested_shapes {
        if test_shape.colides(rest_shape) {
            return false;
        }
    }
    true
}
