use std::io::{self, Read};


fn main() -> io::Result<()> {

    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut floor = 0;
    let mut enters_basement_at = None;

    for (idx, ch) in input.char_indices() {
        floor += match ch {
            '(' => 1,
            ')' => -1,
            _ => 0,
        };
        if enters_basement_at.is_none() && floor == -1 {
            enters_basement_at = Some(idx + 1);
        }
    }

    println!("Floor {}", floor);

    if let Some(idx) = enters_basement_at {
        println!("Santa enters the basement at {}", idx);
    } else {
        println!("Santa never goes to the basement.");
    }

    Ok(())
}
