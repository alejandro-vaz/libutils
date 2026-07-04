//^
//^ HEAD
//^

//> HEAD -> CONSOLE
use libutils_console::{
    Descriptor, 
    Synchronization
};

//> HEAD -> SUPER
use super::TERMINAL;

//> HEAD -> DIFF
use libutils_diff::Diff;

//> HEAD -> ISSUE
use libutils_issue::{
    Issue,
    Severity
};

//> HEAD -> STD
use std::{
    fs::File, 
    io::{
        Read, 
        Write, 
        stdout
    }
};


//^
//^ ACTIONREQUIRED
//^

//> ACTIONREQUIRED -> STRUCT
pub struct ActionRequired;

//> ACTIONREQUIRED -> SYNCHRONIZATION
impl Synchronization for ActionRequired {
    #[inline]
    fn sync(self) -> () {
        let mut content = TERMINAL.layout.read(|layout| layout.iter().map(ToString::to_string).collect::<Vec<String>>()).join("\n\n");
        content.push('\n');
        TERMINAL.output.write(|output| {
            let mut lock = stdout().lock();
            lock.write(<Diff as Into<Vec<u8>>>::into(Diff::new(
                output.as_bytes(), 
                content.as_bytes()
            )).as_ref()).unwrap();
            lock.flush().unwrap();
            *output = content;
        });
    }
}


//^
//^ FILEDESCRIPTOR
//^

//> FILEDESCRIPTOR -> STRUCT
pub struct FileDescriptor {
    pub file: File
}

//> FILEDESCRIPTOR -> DESCRIPTOR
impl Descriptor for FileDescriptor {
    #[inline]
    fn read_bytes(&mut self) -> Result<Vec<u8>, Issue> {
        let mut buffer = Vec::new();
        match self.file.read(&mut buffer) {
            Ok(_) => Ok(buffer),
            Err(error) => Err(Issue {
                name: "Failed to read file as binary",
                description: Some(error.to_string()),
                severity: Severity::Error
            })
        }
    }
    #[inline]
    fn write_bytes(&mut self, content: &[u8]) -> Result<(), Issue> {return match self.file.write(content) {
        Ok(_) => Ok(()),
        Err(error) => Err(Issue {
            name: "Failure writing to file",
            description: Some(error.to_string()),
            severity: Severity::Error
        })
    }}
}