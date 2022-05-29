use std::io::{self, Read};
use std::str::FromStr;

fn paper_for_box(a: u32, b: u32, c: u32) -> u32 {
    a * b * 3 + (b * c + a * c) * 2
}

fn ribbon_for_box(a: u32, b: u32, c: u32) -> u32 {
    (a + b) * 2 + a * b * c
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut total_paper = 0;
    let mut total_ribbon = 0;

    for line in input.lines() {
        let mut dims = line
            .split("x")
            .map(|x| u32::from_str(x).unwrap())
            .collect::<Vec<u32>>();
        dims.sort_unstable();
        total_paper += paper_for_box(dims[0], dims[1], dims[2]);
        total_ribbon += ribbon_for_box(dims[0], dims[1], dims[2]);
    }

    println!("Total paper required: {}", total_paper);
    println!("Total ribbon required: {}", total_ribbon);

    Ok(())
}
