mod advent1;
mod advent10;
mod advent11;
mod advent12;
mod advent13;
mod advent14;
mod advent15;
mod advent16;
mod advent2;
mod advent3;
mod advent4;
mod advent5;
mod advent6;
mod advent7;
mod advent8;
mod advent9;
mod helper;

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
        advent5::advent5(),
        advent5::advent5_2(),
        advent6::advent6(),
        advent6::advent6_2(),
        advent7::advent7(),
        advent7::advent7_2(),
        advent8::advent8(),
        advent8::advent8_2(),
        advent9::advent9(),
        advent9::advent9_2(),
        advent10::advent10(),
        advent10::advent10_2(),
        advent11::advent11(),
        "advent11::advent11_2(), //lasts too long:)".to_string(),
        advent12::advent12(),
        "advent12::advent12_2(), // lasts too long:)".to_string(),
        advent13::advent13(),
        advent13::advent13_2(),
        advent14::advent14(),
        advent14::advent14_2(),
        "advent15::advent15(), // lasts too long:)".to_string(),
        "advent15::advent15_2(), // lasts too long:)".to_string(),
        "advent16::advent16(), // lasts too long:)".to_string(),
        advent16::advent16_2(),
    ];
    println!();
    for (i, res) in results.iter().enumerate() {
        println!("Result {}: {}", i + 1, res);
    }
    return Ok(());
}
