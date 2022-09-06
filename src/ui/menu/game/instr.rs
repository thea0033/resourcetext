use crate::{
    instr::{parse_options, InstrLocation},
    save::Package,
    ui::menu::{config::Config, constants, grab_menu_res, options::OptionTable, MenuResult},
};

impl Package {
    pub fn instr_menu(&mut self, config: &mut Config, loc: InstrLocation) {
        loop {
            let others = self.dir.get_from_loc(&loc).display(loc.obj, &self);
            let numbered = self.dir.get_from_loc(&loc).display_options(loc.obj, &self);
            let table: OptionTable = OptionTable::new(others, numbered, config.context.grab(constants::DEFAULT)); // change later
            match grab_menu_res(&table, config, self) {
                MenuResult::Exit => break,
                MenuResult::Copy => todo!(),
                MenuResult::Paste => todo!(),
                MenuResult::Enter(val) => parse_options(val, self, config, loc.clone()),
                _ => {}
            }
        }
    }
}
