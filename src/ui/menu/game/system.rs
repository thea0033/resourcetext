use crate::ui::menu::{constants, grab_menu_res, options::OptionTable, MenuResult};

use crate::{save::Package, systems::system_id::SystemID, ui::menu::Config};
impl Package {
    pub fn system_menu(&mut self, config: &mut Config, id: SystemID) {
        loop {
            let options: Vec<String> = self.sys.get_system(id).display(&self.sys.get_object_names(), &self.sys);
            let table = OptionTable::new(String::new(), options, config.context.grab(constants::SYSTEM_CONTEXT));
            let res: MenuResult = grab_menu_res(&table, config, self);
            match res {
                MenuResult::Continue => continue,
                MenuResult::Exit => break,
                MenuResult::Copy => self.copy_in_system(id),
                MenuResult::Paste => self.paste_in_system(id),
                MenuResult::Enter(val) => {
                    self.object_menu(config, self.sys.get_system(id).get_objs()[val]);
                }
                MenuResult::New => self.new_in_system(id),
                MenuResult::Remove => self.remove_in_system(id),
            }
        }
    }
    pub fn copy_in_system(&mut self, id: SystemID) {
        unimplemented!()
    }
    pub fn paste_in_system(&mut self, id: SystemID) {
        unimplemented!()
    }
    pub fn new_in_system(&mut self, id: SystemID) {
        unimplemented!()
    }
    pub fn remove_in_system(&mut self, id: SystemID) {
        unimplemented!()
    }
}
