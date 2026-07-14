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