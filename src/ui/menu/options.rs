use std::cmp::min;

use super::{keys::Keys, InputResult};

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
        if self.pages > 1 {
            println!(
                "Showing options {} to {} of {} (page {} of {})",
                page * 10 + 1,
                min((page + 1) * 10, self.numbered.len()),
                self.numbered.len(),
                page + 1,
                self.pages
            );
            if k.visible(InputResult::Up as usize) {
                println!("{}. Go to the next page", k.key(InputResult::Up as usize));
            }
            if k.visible(InputResult::Down as usize) {
                println!("{}. Go to the previous page", k.key(InputResult::Down as usize));
            }
        }
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
    pub fn len(&self) -> usize {
        self.numbered.len()
    }
}