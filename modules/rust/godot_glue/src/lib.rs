mod gdextension;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("hi");
        panic!("compiler decided life wasn't worth it anymore :(");
    }
}
