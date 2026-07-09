//^
//^ HEAD
//^

//> HEAD -> NO STD
#![no_std]

//> HEAD -> CRATES
extern crate alloc;

//> HEAD -> MODULES
mod locales;
#[cfg(test)]
mod tests;

//> HEAD -> LOCALES
use locales::Locales;

//> HEAD -> ALLOC
use alloc::{
    vec::Vec,
    string::{
        String,
        ToString
    },
    format
};


//^
//^ REDUCER
//^

//> REDUCER -> FUNCTION
pub fn reduce(ebnf: &str) -> String {
    let mut rules = Vec::new();
    let mut counter = 0;
    let mut locales = Locales::default();
    for line in ebnf.lines().map(|line| line.trim()).filter(|line| !line.is_empty()) {
        let (rule, mut production) = line.split_once(':').map(|(first, second)| (
            first.trim(), 
            second.trim().to_string()
        )).unwrap();
        expand(&mut production, &mut locales, &mut counter, &mut rules);
        rules.push(format!("{rule}: {production}"));
    };
    rules.sort_by_key(|string| if string.starts_with('$') {
        string.strip_prefix('$').unwrap().split_once(':').unwrap().0.parse().unwrap()
    } else {0});
    return rules.join("\n");
}

//> REDUCER -> EXPAND
fn expand(production: &mut String, locales: &mut Locales, counter: &mut usize, rules: &mut Vec<String>) -> () {
    while let Some(hit) = locales.collapse.find(production.as_str()) {
        let mut inside = production[hit.start() + 1 .. hit.end() - 1].to_string();
        let symbol = *locales.parentheses.entry(inside.clone()).or_insert_with(|| {
            *counter += 1;
            *counter
        });
        expand(&mut inside, locales, counter, rules);
        rules.push(format!("${symbol}: {inside}"));
        *production = format!("{}${symbol}{}", &production[..hit.start()], &production[hit.end()..]);
    }
    while let Some(hit) = locales.postfix.captures(production.as_str()) {
        let atom = hit.name("atom").unwrap().as_str();
        let operator = hit.name("operator").unwrap().as_str().chars().next().unwrap();
        let symbol = *match operator {
            '+' => &mut locales.more,
            '*' => &mut locales.multiple,
            '?' => &mut locales.optional,
            _ => unreachable!()
        }.entry(atom.to_string()).or_insert_with(|| {
            *counter += 1;
            *counter
        });
        rules.push(match operator {
            '+' => format!("${symbol}: {atom} ${symbol} | {atom}"),
            '*' => format!("${symbol}: {atom} ${symbol} |"),
            '?' => format!("${symbol}: {atom} |"),
            _ => unreachable!()
        });
        let group = hit.get(0).unwrap();
        *production = format!("{}${symbol}{}", &production[..group.start()], &production[group.end()..]);
    }
}