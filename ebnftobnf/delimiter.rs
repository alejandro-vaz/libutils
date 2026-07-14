//> SETTINGS -> DELIMITER
#[derive(Clone, Copy)]
pub enum Delimiter {
    ColonSpace,
    SpaceThinArrowSpace,
    SpaceFatArrowSpace,
    SpaceDoubleColonEqualSpace
} const impl From<Delimiter> for &'static str {
    fn from(value: Delimiter) -> Self {return match value {
        Delimiter::ColonSpace => ": ",
        Delimiter::SpaceDoubleColonEqualSpace => " ::= ",
        Delimiter::SpaceFatArrowSpace => " => ",
        Delimiter::SpaceThinArrowSpace => " -> "
    }}
}