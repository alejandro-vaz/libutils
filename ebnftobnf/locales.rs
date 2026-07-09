//^
//^ HEAD
//^

//> HEAD -> HASHBROWN
use hashbrown::HashMap as Map;

//> HEAD -> ALLOC
use alloc::string::String;

//> HEAD -> REGEX
use regex::Regex;


//^
//^ LOCALES
//^

//> LOCALES -> STRUCT
pub struct Locales {
    pub parentheses: Map<String, usize>,
    pub more: Map<String, usize> ,
    pub multiple: Map<String, usize>,
    pub optional: Map<String, usize>,
    pub collapse: Regex,
    pub postfix: Regex
}

//> LOCALES -> DEFAULT
impl Default for Locales {
    fn default() -> Self {return Self {
        parentheses: Map::new(),
        more: Map::new(),
        multiple: Map::new(),
        optional: Map::new(),
        collapse: Regex::new(r"\(([^()]*)\)").unwrap(),
        postfix: Regex::new(r"(?P<atom>[A-Za-z0-9$]+)(?P<operator>[*+?])").unwrap()
    }}
}