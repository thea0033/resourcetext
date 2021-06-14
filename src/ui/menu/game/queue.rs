use crate::{
    instr::{all_instrs, queue::QueueID, Instr, InstrID, InstrLocation},
    save::Package,
    systems::object_id::ObjectID,
    ui::menu::{config::Config, constants, grab_menu_res, options::OptionTable, MenuResult},
};

impl Package {
    pub fn new_instr(&mut self, config: &mut Config) -> Option<Instr> {
        let table: OptionTable = OptionTable::new(
            "What instruction do you want to create?".to_string(),
            all_instrs(),
            config.context.grab(constants::SELECT),
        );
        let id = self.generic_select(config, &table, None, |x| x)?;
        self.instr_match(config, id)
    }
    pub fn instr_match(&mut self, config: &mut Config, option: usize) -> Option<Instr> {
        match option {
            0 => Some(Instr::Move(self.select_location(config))),
            1 => Some(Instr::Jump(self.select_system(config)?)),
            2 => {
                let temp = self.select_system(config)?;
                Some(Instr::Transfer(self.select_resources(config, None), self.select_object(config, temp)?))
            }
            3 => {
                let temp = self.select_system(config)?;
                Some(Instr::Transfer(self.select_resources(config, None), self.select_object(config, temp)?))
            }
            4 => {
                let temp = self.select_system(config)?;
                let temp2 = self.select_object(config, temp)?;
                Some(Instr::MoveTo(temp2))
            }
            5 => Some(Instr::If(
                self.select_condition(config),
                Box::new(self.new_instr(config).unwrap_or(Instr::Fail)),
                Box::new(self.new_instr(config).unwrap_or(Instr::Fail)),
            )),
            6 => Some(Instr::All(vec![])),
            7 => Some(Instr::GoTo(InstrID::new(
                config
                    .buffer
                    .get_flush("Where do you want to go in this instruction queue?", "Please enter a number"),
            ))),
            8 => {
                let (id, amt) = self.select_recipes(config, None)?;
                Some(Instr::PerformRecipe(id, amt))
            }
            9 => {
                let (id, amt) = self.select_components(config, None)?;
                Some(Instr::InstallComponent(id, amt))
            }
            10 => {
                let (id, amt) = self.select_components(config, None)?;
                Some(Instr::RemoveComponent(id, amt))
            }
            11 => Some(Instr::Sticky),
            12 => Some(Instr::End),
            13 => Some(Instr::Fail),
            _ => None,
        }
    }
    pub fn queue_menu(&mut self, config: &mut Config, obj: ObjectID, id: QueueID) {
        loop {
            let temp = self.dir.get(obj).queue(id).display(obj, &self);
            let table: OptionTable = OptionTable::new(String::new(), temp, config.context.grab(constants::SELECT));
            match grab_menu_res(&table, config, self) {
                MenuResult::Continue => (),
                MenuResult::Exit => break,
                MenuResult::Copy => self.copy_in_queue(config),
                MenuResult::Paste => self.paste_in_queue(config),
                MenuResult::Enter(val) => self.instr_menu(config, InstrLocation::new(obj, id, InstrID::new(val))),
                MenuResult::New => self.new_in_queue(config, obj, id),
                MenuResult::Remove => self.remove_in_queue(config, obj, id),
            }
        }
    }
    fn copy_in_queue(&mut self, config: &mut Config) {
        unimplemented!()
    }
    fn paste_in_queue(&mut self, config: &mut Config) {
        unimplemented!()
    }
    fn new_in_queue(&mut self, config: &mut Config, obj: ObjectID, queue: QueueID) {
        self.message(config, "You will now be prompted to select a position for the new instruction.");
        let pos = self.select_instr(config, obj, queue, true);
        if let Some(pos) = pos {
            if let Some(val) = self.new_instr(config) {
                self.dir.get_mut(obj).queue_mut(queue).ins(val, pos);
                self.message(config, "Instruction successfully created!");
            } else {
                self.message(config, "Instruction creation successfully aborted!");
            }
        } else {
            self.message(config, "Instruction creation successfully aborted!");
        }
    }
    fn remove_in_queue(&mut self, config: &mut Config, obj: ObjectID, queue: QueueID) {
        let numbered = self.dir.get(obj).queue(queue).display(obj, &self);
        let table = OptionTable::new(
            "Which queue do you want to remove?".to_string(),
            numbered,
            config.context.grab(constants::SELECT),
        );
        if let Some(pos) = self.generic_select(config, &table, None, |x| x) {
            self.dir.get_mut(obj).queue_mut(queue).rmv(pos);
            self.message(config, "Queue successfully removed!");
        } else {
            self.message(config, "Queue removal aborted!");
        }
    }
    pub fn select_instr(&mut self, config: &mut Config, obj: ObjectID, queue: QueueID, end_allowed: bool) -> Option<QueueID> {
        let queue = self.dir.get(obj).queue(queue);
        let mut positions = queue.display(obj, &self);
        if end_allowed {
            positions.push("Add on end".to_string());
        }
        let table = OptionTable::new(
            "What instruction do you want to select?".to_string(),
            positions,
            config.context.grab(constants::SELECT),
        );
        self.generic_select(config, &table, None, |x| QueueID::new(x))
    }
}
