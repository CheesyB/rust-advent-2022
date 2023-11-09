use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, u32},
    multi::separated_list1,
    sequence::preceded,
    IResult,
};

//Valve OY has flow rate=0; tunnels lead to valves XG, ZB
pub fn parse_line(line: &str) -> IResult<&str, (&str, u32, Vec<&str>)> {
    let (next, name) = preceded(tag("Valve "), alpha1)(line)?;
    let (next, flow_rate) = preceded(tag(" has flow rate="), u32)(next)?;
    let (next, _) = alt((
        tag("; tunnel leads to valve "),
        tag("; tunnels lead to valves "),
    ))(next)?;
    let (next, paths) = separated_list1(tag(", "), alpha1)(next)?;
    IResult::Ok((next, (name, flow_rate, paths)))
}
