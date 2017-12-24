#[derive(Debug)]
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
