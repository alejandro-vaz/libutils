//^
//^ HEAD
//^

//> HEAD -> FEATURES
#![feature(default_field_values)]

//> HEAD -> EBNFTOBNF
use ebnftobnf::{
    reduce,
    Settings
};


//^
//^ TESTS
//^

//> TESTS -> INVARIANT
#[test]
fn invariant() -> () {
    let input = "A: B";
    assert_eq!(reduce(input, Settings {..}), input);
}

//> TESTS -> OPTIONAL
#[test]
fn optional() -> () {
    let input = "A: B?";
    assert_eq!(reduce(input, Settings {..}), r"$1: B | 
A: $1")
}

//> TESTS -> MULTIPLE
#[test]
fn multiple() -> () {
    let input = "% hello
A: B*";
    assert_eq!(reduce(input, Settings {
        comment_start_character: '%',
        ..
    }), r"% hello
$1: B $1 | 
A: $1")
}

//> TESTS -> MORE
#[test]
fn more() -> () {
    let input = "A: B+";
    assert_eq!(reduce(input, Settings {
        temporal_production_character: '&',
        ..
    }), r"&1: B &1 | B
A: &1")
}

//> TESTS -> BIG
#[test]
fn big() -> () {
    let input = "A -> (B C+)*";
    assert_eq!(reduce(input, Settings {
        delimiter: " -> ",
        single_variant_per_line: true,
        ..
    }), "$2 -> C $2\n$2 -> C\n$1 -> B $2\n$3 -> $1 $3\n$3 -> \nA -> $3");
}