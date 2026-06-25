//^
//^ SEVERITY
//^

//> SEVERITY -> ENUM
#[derive(Clone, Copy, Debug)]
pub enum Severity {
    Warning,
    Error,
    Critical
}