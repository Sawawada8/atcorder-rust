use std::io::{self, Error};

fn main() -> Result<(), Error> {
    let h = i()?;
    let hh = i()?;
    println!("{}", h - hh);

    Ok(())
}

/// i
fn i() -> Result<isize, Error> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    let a: isize = buffer.trim().parse().unwrap();

    Ok(a)
}

/// s
#[allow(dead_code)]
fn s() -> Result<String, Error> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    let a: String = buffer.trim().parse().unwrap();

    Ok(a)
}


/// i i i
#[allow(dead_code)]
fn o_i_array() -> Result<Vec<isize>, Error> {
    let mut buffer  = String::new();
    io::stdin().read_line(&mut buffer)?;

    let a: Vec<isize> = buffer.trim().split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect();
    Ok(a)
}

/// s s s
#[allow(dead_code)]
fn o_s_array() -> Result<Vec<String>, Error> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;

    let a: Vec<String> = buffer.trim().split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect();
    Ok(a)
}

/// s s s
/// s s s
/// s s s
#[allow(dead_code)]
fn m_s_array(count: i32) -> Result<Vec<Vec<String>>, Error> {
    let mut result = Vec::new();
    for _n in 0..count {
        result.push(o_s_array()?);
    }

    Ok(result)
}

/// i i i
/// i i i
/// i i i
#[allow(dead_code)]
fn m_i_array(count: i32) -> Result<Vec<Vec<isize>>, Error> {
    let mut result = Vec::new();
    for _n in 0..count {
        result.push(o_i_array()?);
    }

    Ok(result)
}
