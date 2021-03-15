use std::cmp::min;

use super::keys::Keys;

use crate::ui::io::ansi;

pub struct OptionTable {
    others: String,               // Printed first
    context: Vec<Option<String>>, // Printed second
    numbered: Vec<String>,        // Printed last
    pages: usize,
}
impl OptionTable {
    pub fn print(&self, page: usize, k: &Keys) {
        println!("{}{}{}\n", ansi::RESET, self.others, ansi::RESET);
        for (i, line) in self.context.iter().enumerate() {
            if k.visible(i) {
                if let Some(line) = line {
                    println!("{}{}. {}{}", ansi::RESET, k.key(i), line, ansi::RESET);
                }
            }
        }
        println!();
        for i in (page * 10)..min((page + 1) * 10, self.numbered.len()) {
            println!("{}{}. {}{}", ansi::RESET, i % 10, self.numbered[i], ansi::RESET);
        }
    }
    pub fn new(others: String, numbered: Vec<String>, context: Vec<Option<String>>) -> OptionTable {
        OptionTable {
            context,
            pages: (numbered.len() + 9) / 10,
            numbered,
            others,
        }
    }

    /// Get a reference to the option table's pages.
    pub fn pages(&self) -> &usize {
        &self.pages
    }
}
