#![allow(unused)]
fn some_func() -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert!(some_func());
    }
}
