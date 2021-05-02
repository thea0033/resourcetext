use crate::{instr::{Instr, queue::{Queue, QueueID}}, save::Package, systems::object_id::ObjectID, ui::menu::{config::Config, grab_menu_res, options::OptionTable}};
use crate::ui::menu::constants;

impl Package {
    pub fn instrs_menu(&mut self, config: &mut Config, obj: ObjectID) {
        loop {
            let instrs = self.dir.instrs_mut(obj);
            let table:OptionTable = OptionTable::new(String::new(), instrs.display(), config.context.grab(constants::SELECT));
            match grab_menu_res(&table, config, self) {
                crate::ui::menu::MenuResult::Continue => (),
                crate::ui::menu::MenuResult::Exit => break,
                crate::ui::menu::MenuResult::Copy => self.illegal_state(config),
                crate::ui::menu::MenuResult::Paste => {}
                crate::ui::menu::MenuResult::Enter(val) => self.queue_menu(config, obj, QueueID::new(val)),
                crate::ui::menu::MenuResult::New => self.new_in_instrs(config, obj),
                crate::ui::menu::MenuResult::Remove => self.remove_in_instrs(config, obj)
            }
        }
    }
    fn select_instr(&mut self, config: &mut Config, obj: ObjectID, end_allowed:bool) -> Option<QueueID> {
        let instrs = self.dir.instrs_mut(obj);
        let mut positions = instrs.display();
        if end_allowed {
            positions.push("Add on end".to_string());
        }
        let table = OptionTable::new("What queue do you want to select?".to_string(), positions, config.context.grab(constants::SELECT));
        self.generic_select(config, &table, None, |x| QueueID::new(x))
    }
    fn remove_in_instrs(&mut self, config: &mut Config, obj: ObjectID) {
        let remove_pos = self.select_instr(config, obj, false);
        if let Some(val) = remove_pos {
            self.dir.instrs_mut(obj).rmv(val);
        }
    }
    fn new_in_instrs(&mut self, config: &mut Config, obj: ObjectID) {
        let insert_pos = self.select_instr(config, obj, true);
        if let Some(val) = insert_pos {
            let (queue, name) = self.new_queue(config, obj, val);
            self.dir.instrs_mut(obj).insert(val, queue, name);
        } else {
            self.aborted(config);
        }
    }
    fn new_queue(&mut self, config: &mut Config, obj: ObjectID, id: QueueID) -> (Queue, String) {
        println!("Is this queue to be only used once?");
        let delete_after_exe = config.buffer.get_flush("Please enter true or false.");
        println!("What do you want to call this queue?");
        let name = config.buffer.get_flush("Enter a name for the instruction.");
        (Queue::new(delete_after_exe, self.new_instr(config, obj, id).unwrap_or(Instr::Fail)),
        name)
    }
}