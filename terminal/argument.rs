//^
//^ ARGUMENT
//^

//> ARGUMENT -> ENUM
pub enum Argument {
    File(String),
    Alias(Vec<char>),
    Flag(String),
    Setting(String, String),
    Target(String),
    Unknown(String)
} impl From<String> for Argument {
    fn from(value: String) -> Self {return match value {
        ref setting if let Some(setting) = setting.strip_prefix("--") && let Some((key, value)) = setting.split_once('=') => Argument::Setting(key.to_string(), value.to_string()),
        ref flag if let Some(flag) = flag.strip_prefix("--") => Argument::Flag(flag.to_string()),
        ref alias if let Some(alias) = alias.strip_prefix('-') => Argument::Alias(alias.chars().collect()),
        other => {
            let mut file = Some(false);
            for character in other.chars() {file = match character {
                num if num.is_numeric() => Some(true),
                '/' | '\\' | '.' => Some(true),
                letter if letter.is_alphabetic() => file,
                _ => None
            }};
            match file {
                None => Argument::Unknown(other),
                Some(true) => Argument::File(other),
                Some(false) => Argument::Target(other)
            }
        }
    }}
}