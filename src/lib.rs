mod doz;
mod lambs;

#[derive(Debug, PartialEq, Clone)]
pub enum Valamdo {
    Name(doz::Name),
    Func(lambs::Func),
    Appl(lambs::Func, Box<Valamdo>),
}

#[cfg(test)]
mod tests {
    use super::Valamdo;
    use doz::Name;
    use lambs::Func;
    #[test]
    fn it_works() {
        let name = Name {
            name: String::from("hello_world"),
        };
        let barf = Valamdo::Name(name.clone());
        let funn = Func { name, body: Box::new(barf) };
        let hello = Valamdo::Func(funn.clone());
        let world = Valamdo::Appl(funn, Box::new(hello));
        assert_eq!(world, world);
    }
}
