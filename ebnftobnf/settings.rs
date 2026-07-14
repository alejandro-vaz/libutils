//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::{
    delimiter::Delimiter,
    commentstyle::CommentStyle,
    temporalproductionstyle::TemporalProductionStyle
};


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