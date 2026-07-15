//^
//^ HEAD
//^

//> HEAD -> ALLOC
use alloc::string::String;


//^
//^ CLITYPE
//^

//> CLITYPE -> ENUM
#[derive(Debug)]
pub enum CliType {
    Number(usize),
    Other(String)
}

//> CLITYPE -> FROM STRING
impl From<String> for CliType {
    fn from(value: String) -> Self {
        return if let Ok(number) = value.parse() {
            CliType::Number(number)
        } else {
            CliType::Other(value)
        }
    }
}