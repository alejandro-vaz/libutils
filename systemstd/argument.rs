//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::{
    clitype::CliType,
    ioerror::IoError
};

//> HEAD -> STD
use std::path::PathBuf;


//^
//^ ARGUMENT
//^

//> ARGUMENT -> ENUM
#[derive(Debug, Clone)]
pub enum Argument<'valid> {
    Path {
        buffer: PathBuf
    },
    Alias {
        characters: Vec<char>
    },
    Flag {
        value: &'valid str
    },
    Setting {
        key: &'valid str, 
        value: CliType
    },
    Target {
        to: &'valid str
    }
} 

//> ARGUMENT -> PARSING
impl<'valid> TryFrom<&'valid str> for Argument<'valid> {
    type Error = IoError<'valid>;
    fn try_from(value: &'valid str) -> Result<Self, Self::Error> {return Ok(match value {
        capture if let Some(Some((key, value))) = capture.strip_prefix("--").map(|item| {
            item.split_once('=')
        }) => Argument::Setting {
            key: key, 
            value: CliType::try_from(value)?
        },
        capture if let Some(flag) = capture.strip_prefix("--") => Argument::Flag {
            value: flag
        },
        capture if let Some(alias) = capture.strip_prefix('-') => Argument::Alias {
            characters: alias.chars().collect()
        },
        other => {
            let mut path = false;
            for character in other.chars() {match character {
                num if num.is_numeric() => path = true,
                '/' | '\\' | '.' | ':' => path = true,
                letter if letter.is_alphabetic() => (),
                _ => return Err(IoError::FailureParsingArgument {
                    argument: other
                })
            }};
            if path {Argument::Path {
                buffer: PathBuf::from(other)
            }} else {Argument::Target {
                to: other
            }}
        }
    })}
}