




// just for illustration purposes
fn internal_adder(a: u8, b: u8) -> u8 {
    return a + b;
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}
