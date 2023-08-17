mod advent1;
mod advent2;
mod advent3;
mod advent4;
fn main() -> Result<(), std::io::Error> {
    let results = [
        advent1::advent1(),
        advent1::advent1_2(),
        advent2::advent2(),
        advent2::advent2_2(),
        advent3::advent3(),
        advent3::advent3_2(),
        advent4::advent4(),
        advent4::advent4_2(),
    ];
    println!();
    for (i, res) in results.iter().enumerate() {
        println!("Result {}: {}", i + 1, res);
    }
    return Ok(());
}
