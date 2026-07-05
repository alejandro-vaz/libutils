//^
//^ HEAD
//^

//> HEAD -> CONSOLE
use libutils_console::{
    Descriptor, 
    Synchronization,
    Data
};

//> HEAD -> SUPER
use super::{
    LAYOUT,
    OUTPUT
};

//> HEAD -> DIFF
use libutils_diff::Diff;

//> HEAD -> ISSUE
use libutils_issue::{
    Issue,
    Severity
};

//> HEAD -> STD
use std::{
    fs::{
        File,
        Metadata
    }, 
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
    fn sync(self) -> () {
        let mut content = LAYOUT.read(|layout| layout.iter().map(ToString::to_string).collect::<Vec<String>>()).join("\n\n");
        content.push('\n');
        OUTPUT.write(|output| {
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
    fn metadata(&self) -> Result<impl Data, Issue> {return match self.file.metadata() {
        Ok(metadata) => Ok(FileData {
            metadata: metadata
        }),
        Err(error) => Err(Issue {
            name: "Failed to read size of file",
            description: Some(error.to_string()),
            severity: Severity::Error
        })
    }}
    fn read_bytes(&mut self) -> Result<Vec<u8>, Issue> {
        let mut buffer = Vec::with_capacity(self.metadata()?.size());
        match self.file.read(&mut buffer) {
            Ok(_) => Ok(buffer),
            Err(error) => Err(Issue {
                name: "Failed to read file as binary",
                description: Some(error.to_string()),
                severity: Severity::Error
            })
        }
    }
    fn write_bytes(&mut self, content: &[u8]) -> Result<(), Issue> {return match self.file.write(content) {
        Ok(_) => Ok(()),
        Err(error) => Err(Issue {
            name: "Failure writing to file",
            description: Some(error.to_string()),
            severity: Severity::Error
        })
    }}
}


//^
//^ FILEDATA
//^

//> FILEDATA -> STRUCT
pub struct FileData {
    metadata: Metadata
}

//> FILEDATA -> DATA
impl Data for FileData {
    fn size(&self) -> usize {return self.metadata.len() as usize}
}