//^
//^ STATE
//^

//> STATE -> BASE
pub trait State {
    fn chain(&self) -> Vec<&'static str>;
    fn link<'valid>(&'valid mut self) -> &'valid mut Vec<&'static str>;
}

//> STATE -> DERIVED
pub trait DerivedState<'valid>: State + From<&'valid mut dyn State> {}


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
//^ SAME
//^

//> SAME -> STRUCT
pub struct Same<'this> {
    link: &'this mut Vec<&'static str>
} 

//> SAME -> STATE
impl<'this> State for Same<'this> {
    fn chain(&self) -> Vec<&'static str> {return self.link.clone()}
    fn link<'valid>(&'valid mut self) -> &'valid mut Vec<&'static str> {return self.link}
}

//> SAME -> DERIVED STATE
impl<'valid> DerivedState<'valid> for Same<'valid> {}

//> SAME -> FROM
impl<'valid> From<&'valid mut (dyn State + 'valid)> for Same<'valid> {
    fn from(value: &'valid mut (dyn State + 'valid)) -> Self {return Self {
        link: value.link()
    }}
}


//^
//^ NAME
//^

//> NAME -> STRUCT
pub struct Name<'valid, const NAME: &'static str> {
    link: &'valid mut Vec<&'static str>
} 

//> NAME -> STATE
impl<'this, const NAME: &'static str> State for Name<'this, NAME> {
    fn chain(&self) -> Vec<&'static str> {return self.link.clone()}
    fn link<'valid>(&'valid mut self) -> &'valid mut Vec<&'static str> {return self.link}
} 

//> NAME -> DROP
impl<'valid, const NAME: &'static str> Drop for Name<'valid, NAME> {
    fn drop(&mut self) {self.link.pop();}
}

//> NAME -> DERIVED STATE
impl<'valid, const NAME: &'static str> DerivedState<'valid> for Name<'valid, NAME> {}

//> NAME -> FROM
impl<'valid, const NAME: &'static str> From<&'valid mut (dyn State + 'valid)> for Name<'valid, NAME> {
    fn from(value: &'valid mut (dyn State + 'valid)) -> Self {
        value.link().push(NAME);
        return Self {
            link: value.link()
        }
    }
}