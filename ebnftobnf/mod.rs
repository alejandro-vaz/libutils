//^
//^ HEAD
//^

//> HEAD -> NO STD
#![no_std]

//> HEAD -> DOCS
#![doc = include_str!("README.md")]

//> HEAD -> FEATURES
#![feature(default_field_values)]
#![feature(const_convert)]
#![feature(const_trait_impl)]

//> HEAD -> CRATES
extern crate alloc;

//> HEAD -> MODULES
mod commentstyle;
mod delimiter;
mod locales;
mod settings;
mod temporalproductionstyle;
#[cfg(test)]
mod tests;

//> HEAD -> LOCALES
use locales::Locales;

//> HEAD -> ALLOC
use alloc::{
    string::{
        String,
        ToString
    },
    format
};

//> HEAD -> TYPED_ARENA
use typed_arena::Arena;

//> HEAD -> SETTINGS
pub use settings::Settings;

//> HEAD -> COMMENTSTYLE
pub use commentstyle::CommentStyle;

//> HEAD -> DELIMITER
pub use delimiter::Delimiter;

//> HEAD -> TEMPORALPRODUCTIONSTYLE
pub use temporalproductionstyle::TemporalProductionStyle;

//> HEAD -> HASHBROWN
use hashbrown::HashSet as Set;


//^
//^ REDUCER
//^

//> REDUCER -> FUNCTION
pub fn reduce(ebnf: &str, settings: Settings) -> String {
    let mut rules = Set::new();
    let mut counter = 0;
    let arena = Arena::new();
    let mut locales = Locales::default();
    for line in ebnf.lines().map(|line| line.trim()) {
        if line.is_empty() {
            if settings.keep_empty_lines {rules.insert(line.to_string());}
            continue;
        }
        if line.starts_with(char::from(settings.comment_style)) {
            if settings.keep_comments {rules.insert(line.to_string());}
            continue;
        }
        let (rule, mut production) = line.split_once(
            <&'static str>::from(settings.delimiter)
        ).map(|(first, second)| (
            first.trim(), 
            second.trim().to_string()
        )).unwrap();
        expand(&mut production, &mut locales, &mut counter, &mut rules, &arena, settings);
        rules.insert(format!("{rule}{}{production}", <&'static str>::from(settings.delimiter)));
    };
    rules.insert(format!(
        "{}0{}Start",
        char::from(settings.temporal_production_style),
        <&'static str>::from(settings.delimiter)
    ));
    let mut full = String::new();
    for rule in rules {
        full.push_str(&rule);
        full.push('\n');
    };
    full.pop();
    return full;
}

//> REDUCER -> EXPAND
fn expand<'arena>(
    production: &mut String, 
    locales: &mut Locales<'arena>, 
    counter: &mut usize, 
    rules: &mut Set<String>, 
    arena: &'arena Arena<String>,
    settings: Settings
) -> () {
    while let Some(hit) = locales.collapse.find(production.as_str()) {
        let mut inside = production[hit.start() + 1 .. hit.end() - 1].to_string();
        let symbol = *locales.parentheses.entry(inside.clone()).or_insert_with(|| {
            *counter += 1;
            *counter
        });
        expand(&mut inside, locales, counter, rules, arena, settings);
        rules.insert(format!(
            "{}{symbol}{}{inside}", 
            char::from(settings.temporal_production_style), 
            <&'static str>::from(settings.delimiter)
        ));
        *production = format!(
            "{}{}{symbol}{}", 
            &production[..hit.start()], 
            char::from(settings.temporal_production_style), 
            &production[hit.end()..]
        );
    }
    while let Some(hit) = locales.postfix.captures(production.as_str()) {
        let atom = arena.alloc(hit.name("atom").unwrap().as_str().to_string());
        let operator = hit.name("operator").unwrap().as_str().chars().next().unwrap();
        let symbol = *match operator {
            '+' => &mut locales.more,
            '*' => &mut locales.multiple,
            '?' => &mut locales.optional,
            _ => unreachable!()
        }.entry(atom).or_insert_with(|| {
            *counter += 1;
            *counter
        });
        rules.insert(format!(
            "{}{symbol}{}{atom} {}",
            char::from(settings.temporal_production_style),
            <&'static str>::from(settings.delimiter),
            match operator {
                '+' => format!("{}{symbol} | {atom}", char::from(settings.temporal_production_style)),
                '*' => format!("{}{symbol} |", char::from(settings.temporal_production_style)),
                '?' => format!("|"),
                _ => unreachable!()
            }
        ));
        let group = hit.get(0).unwrap();
        *production = format!(
            "{}{}{symbol}{}", 
            &production[..group.start()], 
            char::from(settings.temporal_production_style), 
            &production[group.end()..]
        );
    }
}