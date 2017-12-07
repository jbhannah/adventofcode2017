use std::fs::File;
use std::io::Read;

pub fn read_input(day: u32) -> String {
    let mut f = File::open(format!("../input/day{:02}.txt", day)).expect("File not found!");
    let mut contents = String::new();

    f.read_to_string(&mut contents).expect(
        "Could not read file!",
    );
    return contents;
}
