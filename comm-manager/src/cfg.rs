use std::fs::File;
use std::io::Read;



// just for illustration purposes
fn internal_adder(a: u8, b: u8) -> u8 {
    return a + b;
}



#[cfg(test)]
mod tests {
    use std::env;

    use super::*;

    #[test]
    fn internal() {
        let path = env::current_dir().unwrap();
        let mut f = File::open("tests/data/cfg_example1.yaml").unwrap();
        let mut content = String::new();
        f.read_to_string(&mut content).unwrap();
        println!("{}", content);
        assert_eq!(4, internal_adder(2, 2));
    }
}
