use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::rc::Rc;

#[derive(Debug, Default)]
pub struct Char {
    pub chr: String,
    pub div: Vec<String>,
    pub freq: u64,
}

#[derive(Debug, Default)]
pub struct Comp {
    pub comp: String,
    pub code: String,
    pub freq: u64,
}

#[derive(Debug, Default)]
pub struct AppData {
    /// A dict to query character metadata
    pub charset: HashMap<String, Rc<RefCell<Char>>>,
    /// A dict to query component metadata
    pub compset: HashMap<String, Rc<RefCell<Comp>>>,
}

impl AppData {
    /// Read mappings data from file
    pub fn init(&mut self) -> super::super::AppResult<()> {
        // filter empty lines and comment lines
        let braces: &[_] = &['{', '}'];
        let square_brackets: &[_] = &['[', ']'];
        let re = regex::Regex::new(r"\{.*?\}|.").unwrap();
        let filter = |&line: &&str| line.len() != 0 && !line.starts_with('#');

        // read mappings, divisions, freqs from file
        fs::read_to_string("./table/smyh_map.txt")?
            .lines()
            .filter(filter)
            .for_each(|line| {
                if let Some((code, comp)) = line.split_once('\t') {
                    if let None = self.compset.get(comp) {
                        let component = Comp {
                            comp: comp.to_string(),
                            code: code.to_string(),
                            freq: 0,
                        };
                        self.compset
                            .insert(comp.to_string(), Rc::new(RefCell::new(component)));
                    } else {
                        // TODO: duplicated components
                    }
                }
            });
        fs::read_to_string("./table/smyh_div.txt")?
            .lines()
            .filter(filter)
            .for_each(|line| {
                if let Some((chr, divs)) = line.split_once('\t') {
                    if let None = self.charset.get(chr) {
                        let metas: Vec<String> = divs
                            .trim_matches(square_brackets)
                            .split(',')
                            .map(|x| x.to_string())
                            .collect();
                        if let Some(divs) = metas.first() {
                            let comps: Vec<String> = re
                                .find_iter(divs)
                                .map(|m| m.as_str().trim_matches(braces).to_string())
                                .collect();
                            let character = Char {
                                chr: chr.to_string(),
                                div: comps,
                                freq: 0,
                            };
                            self.charset
                                .insert(chr.to_string(), Rc::new(RefCell::new(character)));
                        } else {
                            // TODO: invalid division line
                        }
                    } else {
                        // TODO: duplicated characters
                    }
                }
            });
        fs::read_to_string("./table/freq.txt")?
            .lines()
            .filter(filter)
            .for_each(|line| {
                if let Some((chr, freq)) = line.split_once('\t') {
                    if let Ok(freq) = freq.parse::<f64>() {
                        let freq = (freq * 1e8) as u64;
                        if freq == 0 {
                            return;
                        }
                        if let Some(character) = self.charset.get(chr) {
                            let mut character = character.borrow_mut();
                            character.freq += freq;
                            for comp in character.div.iter_mut() {
                                if let Some(component) = self.compset.get(comp) {
                                    let mut component = component.borrow_mut();
                                    component.freq += freq;
                                } else {
                                    // TODO: strange components
                                }
                            }
                        } else {
                            // TODO: strange characters
                        }
                    }
                }
            });

        Ok(())
    }
}
