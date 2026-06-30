//^
//^ STATE
//^

//> STATE -> BASE
pub trait State {
    fn chain(&self) -> Vec<&'static str>;
    fn link<'valid>(&'valid mut self) -> &'valid mut Vec<&'static str>;
}

//> STATE -> DERIVED
pub trait DerivedState<'valid>: State + for<'all> From<&'valid mut dyn State> {}


//^
//^ MAIN
//^

//> MAIN -> STRUCT
pub struct Main {
    pub chain: Vec<&'static str> = Vec::new()
}

//> MAIN -> STATE
impl State for Main {
    fn chain(&self) -> Vec<&'static str> {return self.chain.clone()}
    fn link<'valid>(&'valid mut self) -> &'valid mut Vec<&'static str> {return &mut self.chain}
}


//^
//^ STAY
//^

//> STAY -> STRUCT
pub struct Stay<'this> {
    link: &'this mut Vec<&'static str>
} 

//> STAY -> STATE
impl<'this> State for Stay<'this> {
    fn chain(&self) -> Vec<&'static str> {return self.link.clone()}
    fn link<'valid>(&'valid mut self) -> &'valid mut Vec<&'static str> {return self.link}
}

//> STAY -> DERIVED STATE
impl<'valid> DerivedState<'valid> for Stay<'valid> {}

//> STAY -> FROM
impl<'valid> From<&'valid mut (dyn State + 'valid)> for Stay<'valid> {
    fn from(value: &'valid mut (dyn State + 'valid)) -> Self {return Self {
        link: value.link()
    }}
}


//^
//^ ADD
//^

//> ADD -> STRUCT
pub struct Add<'valid, const NAME: &'static str> {
    link: &'valid mut Vec<&'static str>
} 

//> ADD -> STATE
impl<'this, const NAME: &'static str> State for Add<'this, NAME> {
    fn chain(&self) -> Vec<&'static str> {return self.link.clone()}
    fn link<'valid>(&'valid mut self) -> &'valid mut Vec<&'static str> {return self.link}
} 

//> ADD -> DROP
impl<'valid, const NAME: &'static str> Drop for Add<'valid, NAME> {
    fn drop(&mut self) {self.link.pop();}
}

//> ADD -> DERIVED STATE
impl<'valid, const NAME: &'static str> DerivedState<'valid> for Add<'valid, NAME> {}

//> ADD -> FROM
impl<'valid, const NAME: &'static str> From<&'valid mut (dyn State + 'valid)> for Add<'valid, NAME> {
    fn from(value: &'valid mut (dyn State + 'valid)) -> Self {
        value.link().push(NAME);
        return Self {
            link: value.link()
        }
    }
}