//^
//^ SEVERITY
//^

//> SEVERITY -> ENUM
#[derive(Hash, Debug)]
pub enum Severity {
    Warning,
    Error,
    Critical
} const impl Into<&'static str> for &Severity {
    fn into(self) -> &'static str {return match self {
        Severity::Critical => "critical",
        Severity::Error => "error",
        Severity::Warning => "warning"
    }}
}