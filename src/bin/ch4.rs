#![allow(unused)]

fn main() {
    println!("Hello!");
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
}
