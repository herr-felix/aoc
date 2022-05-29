use std::io::{self, Read};


fn main() -> io::Result<()> {

    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut floor = 0;
    let mut enters_basement_at = None;

    for (idx, ch) in input.char_indices() {
        match ch {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => (),
        };
        if enters_basement_at == None && floor == -1 {
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
