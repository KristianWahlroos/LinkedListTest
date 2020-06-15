#[derive(Debug)]
pub enum List<T> {
    Nil,
    Cons(T, Box<List<T>>),
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn test() {
    let list: List<i32> = List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))));
    println!("{:?}", list);    
    }
}