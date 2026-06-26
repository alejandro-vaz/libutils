//^
//^ SEVERITY
//^

//> SEVERITY -> ENUM
pub enum Severity {
    Warning,
    Error,
    Critical
} impl const Into<&'static str> for &Severity {
    fn into(self) -> &'static str {return match self {
        Severity::Critical => "critical",
        Severity::Error => "error",
        Severity::Warning => "warning"
    }}
}