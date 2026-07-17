//^
//^ HEAD
//^

//> HEAD -> CORE
use core::marker::ConstParamTy_ as ConstantParameter;


//^
//^ OPENMODE
//^

//> OPENMODE -> STRUCT
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum OpenMode {
    Read,
    Overwrite,
    Append
}

//> OPENMODE -> CONSTANTPARAMETER
impl ConstantParameter for OpenMode {}

//> OPENMODE -> WRITEABLE
pub const fn writeable<const MODE: OpenMode>() -> bool {return match MODE {
    OpenMode::Read => false,
    _ => true
}}