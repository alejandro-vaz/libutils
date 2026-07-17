//^
//^ HEAD
//^

//> HEAD -> STD
use std::{
    fs::{
        File as StdFile, 
        exists
    }, 
    path::PathBuf
};

//> HEAD -> SUPER
use super::{
    descriptor::Descriptor,
    ioerror::IoError,
    openmode::{
        OpenMode,
        writeable
    }
};


//^ 
//^ PATH
//^

//> PATH -> STRUCT
pub struct Path {
    pub name: PathBuf
}

//> PATH -> IMPLEMENTATION
impl Path {
    pub fn exists<'valid>(&'valid self) -> Result<bool, IoError<'valid>> {
        return exists(&self.name).map_err(|error| IoError::CantKnowIfPathExists {
            path: &self.name, 
            error: error 
        })
    }
    pub fn file<const MODE: OpenMode>(
        self, 
        handling: Option<bool>
    ) -> Result<Descriptor<{writeable::<MODE>()}>, IoError<'static>> {
        let mut options = StdFile::options();
        match MODE {
            OpenMode::Read => options.read(true),
            OpenMode::Overwrite => options.write(true).truncate(true),
            OpenMode::Append => options.write(true)
        };
        match handling {
            None => options.create(false).create_new(false),
            Some(false) => options.create(true),
            Some(true) => options.create_new(true)
        };
        return match options.open(&self.name) {
            Ok(stdfile) => Ok(Descriptor {
                stdfile: stdfile
            }),
            Err(error) => Err(IoError::CouldntOpenFile {
                error: error, 
                name: self.name
            })
        }
    }
}