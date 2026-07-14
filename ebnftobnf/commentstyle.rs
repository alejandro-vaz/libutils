

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