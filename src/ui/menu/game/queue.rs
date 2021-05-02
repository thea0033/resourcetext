use crate::{instr::{Instr, InstrID, queue::QueueID}, save::Package, systems::object_id::ObjectID, ui::menu::{MenuResult, config::Config, constants, context::{self, Context}, grab_menu_res, options::OptionTable}};

impl Package {
    pub fn new_instr(&mut self, config: &mut Config, obj: ObjectID, id: QueueID) -> Option<Instr> {
        let numbered = self.dir.instrs_mut(obj).queue(id).display(obj, &self.sys, &self.rss, &self.cmp);
        let table:OptionTable = OptionTable::new("What instruction do you want to create?".to_string(), numbered, config.context.grab(constants::SELECT));
        let id = self.generic_select(config, &table, None, |x| x)?;
        Some(self.instr_match(config, id))
    }
    pub fn instr_match(&mut self, config: &mut Config, option: usize) -> Instr {
        match option {
            0 => Instr::Move(self.select_location(config)),
            1 => Instr::Jump(self.select_system(config)),
            2 => Instr::Transfer(self.select_resources(config))
        }
    }
    pub fn queue_menu(&mut self, config: &mut Config, obj: ObjectID, id: QueueID) {
        loop {
            let temp = self.dir.instrs_mut(obj).queue(id).display(obj, &self.sys, &self.rss, &self.cmp);
            let table:OptionTable = OptionTable::new(String::new(), temp, config.context.grab(constants::SELECT));
            match grab_menu_res(&table, config, self) {
                MenuResult::Continue => (),
                MenuResult::Exit => break,
                MenuResult::Copy => self.copy_in_queue(config),
                MenuResult::Paste => self.paste_in_queue(config),
                MenuResult::Enter(val) => self.instr_menu(obj, id, InstrID::new(val)),
                MenuResult::New => self.new_in_queue(config),
                MenuResult::Remove => self.remove_in_queue(config)
            }
        }
    }
    pub fn copy_in_queue(&mut self, config: &mut Config) {
        unimplemented!()
    }
    pub fn paste_in_queue(&mut self, config: &mut Config) {
        unimplemented!()
    }
    pub fn new_in_queue(&mut self, config: &mut Config) {

    }
    pub fn remove_in_queue(&mut self, config: &mut Config) {

    }
}
