use nom::{
    bytes::complete::tag,
    character::complete::i64,
    sequence::{preceded, separated_pair},
    IResult,
};

use super::domain::Coord;

pub fn parse_line(line: &str) -> (Coord, Coord) {
    let coords: IResult<&str, ((i64, i64), (i64, i64))> = separated_pair(
        preceded(
            tag("Sensor at "),
            separated_pair(
                preceded(tag("x="), i64),
                tag(", "),
                preceded(tag("y="), i64),
            ),
        ),
        tag(": closest beacon is at "),
        separated_pair(
            preceded(tag("x="), i64),
            tag(", "),
            preceded(tag("y="), i64),
        ),
    )(line);
    coords
        .map(|(_, c)| (Coord(c.0 .0, c.0 .1), Coord(c.1 .0, c.1 .1)))
        .ok()
        .unwrap()
}
