//^
//^ HEAD
//^

//> HEAD -> ISSUING
use issuing::Issue;

//> HEAD -> CORE
use core::io::Error;

//> HEAD -> STD
use std::string::FromUtf8Error;


//^
//^ STDIOERROR
//^

//> STDIOERROR -> ENUM
#[derive(Debug)]
pub enum IoError {
    CouldntOpenFile {
        error: Error
    },
    CouldntReadMetadata {
        error: Error
    },
    CouldntReadFile {
        error: Error
    },
    CouldntWriteToFile {
        error: Error
    },
    FailedToEncodeRead {
        error: FromUtf8Error
    }
}

//> STDIOERROR -> INTO ISSUE
impl Into<Issue> for IoError {
    fn into(self) -> Issue {return match self {
        IoError::CouldntOpenFile {error} => Issue {
            name: "failed to open file",
            description: Some(error.to_string())
        },
        IoError::CouldntReadMetadata {error} => Issue {
            name: "failed to read file metadata",
            description: Some(error.to_string())
        },
        IoError::CouldntReadFile {error} => Issue {
            name: "failed to read file",
            description: Some(error.to_string())
        },
        IoError::CouldntWriteToFile {error} => Issue {
            name: "failed to write to file",
            description: Some(error.to_string())
        },
        IoError::FailedToEncodeRead {error} => Issue {
            name: "failed to encode file to UTF-8",
            description: Some(error.to_string())
        }
    }}
}
