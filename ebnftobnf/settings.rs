//^
//^ SETTINGS
//^

//> SETTINGS -> STRUCT
#[derive(Clone, Copy)]
pub struct Settings {
    pub keep_empty_lines: bool = true,
    pub keep_comments: bool = true,
    pub comment_style: CommentStyle = CommentStyle::Slash,
    pub temporal_production_style: TemporalProductionStyle = TemporalProductionStyle::Dollar,
    pub delimiter: Delimiter = Delimiter::ColonSpace
}

//> SETTINGS -> COMMENTSTYLE
#[derive(Clone, Copy)]
pub enum CommentStyle {
    Hashtag,
    Slash,
    Percentage
} const impl From<CommentStyle> for char {
    fn from(value: CommentStyle) -> Self {return match value {
        CommentStyle::Slash => '/',
        CommentStyle::Hashtag => '#',
        CommentStyle::Percentage => '%'
    }}
}

//> SETTINGS -> TEMPORALPRODUCTIONSTYLE
#[derive(Clone, Copy)]
pub enum TemporalProductionStyle {
    Dollar,
    AtSymbol,
    Ampersand,
    Exclamation
} const impl From<TemporalProductionStyle> for char {
    fn from(value: TemporalProductionStyle) -> Self {return match value {
        TemporalProductionStyle::Ampersand => '&',
        TemporalProductionStyle::AtSymbol => '@',
        TemporalProductionStyle::Dollar => '$',
        TemporalProductionStyle::Exclamation => '!'
    }}
}

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