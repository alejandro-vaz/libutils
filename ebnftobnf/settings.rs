//^
//^ SETTINGS
//^

//> SETTINGS -> STRUCT
#[derive(Clone, Copy)]
pub struct Settings<'valid> {
    pub keep_empty_lines: bool = true,
    pub keep_comments: bool = true,
    pub comment_start_character: char = '#',
    pub temporal_production_character: char = '$',
    pub delimiter: &'valid str = ": ",
    pub augment_start: Option<&'valid str> = None,
    pub single_variant_per_line: bool = false,
    pub variant_separator: &'valid str = " | "
}