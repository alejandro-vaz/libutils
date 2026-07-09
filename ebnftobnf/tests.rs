//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::reduce;


//^
//^ TESTS
//^

//> TESTS -> INVARIANT
#[test]
fn invariant() -> () {
    let input = "A: B";
    assert_eq!(reduce(input), input);
}

//> TESTS -> OPTIONAL
#[test]
fn optional() -> () {
    let input = "A: B?";
    assert_eq!(reduce(input), r"A: $1
$1: B |")
}

//> TESTS -> MULTIPLE
#[test]
fn multiple() -> () {
    let input = "A: B*";
    assert_eq!(reduce(input), r"A: $1
$1: B $1 |")
}

//> TESTS -> MORE
#[test]
fn more() -> () {
    let input = "A: B+";
    assert_eq!(reduce(input), r"A: $1
$1: B $1 | B")
}

//> TESTS -> BIG
#[test]
fn big() -> () {
    let input = "A: (B C+)*";
    assert_eq!(reduce(input), "A: $3\n$1: B $2\n$2: C $2 | C\n$3: $1 $3 |");
}