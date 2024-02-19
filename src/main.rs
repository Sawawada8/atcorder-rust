use std::io::{self, Error};

fn main() -> Result<(), Error> {
    // let a = one_line()?;
    // let a = one_line_array()?;
    let a = multi_line_array(3)?;

    println!("{:?}", a);

    Ok(())
}

/// s
fn one_line() -> Result<String, Error> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    let a: String = buffer.trim().parse().unwrap();

    Ok(a)
}

/// i
fn one_line_int() -> Result<i32, Error> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    let a: i32 = buffer.trim().parse().unwrap();

    Ok(a)
}

///
/// x x x
fn one_line_array() -> Result<Vec<String>, Error> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;

    let a: Vec<String> = buffer.trim().split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect();
    Ok(a)
}

///
/// x x x
/// x x x
/// x x x
fn multi_line_array(count: i32) -> Result<Vec<Vec<String>>, Error> {
    let mut result = Vec::new();
    for n in 0..count {
        result.push(one_line_array()?);
    }

    Ok(result)
}
