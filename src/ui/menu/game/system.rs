use crate::{
    systems::object_id::ObjectID,
    ui::menu::{constants, grab_menu_res, options::OptionTable, MenuResult},
};

use crate::{save::Package, systems::system_id::SystemID, ui::menu::Config};
impl Package {
    pub fn system_menu(&mut self, config: &mut Config, id: SystemID) {
        loop {
            let options: Vec<String> = self.sys.get_system(id).display(&self.sys.get_object_names(), &self.sys);
            let table = OptionTable::new(String::new(), options, config.context.grab(constants::SYSTEM));
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
    fn copy_in_system(&mut self, id: SystemID) {
        unimplemented!()
    }
    fn paste_in_system(&mut self, id: SystemID) {
        unimplemented!()
    }
    fn new_in_system(&mut self, id: SystemID) {
        unimplemented!()
    }
    fn remove_in_system(&mut self, id: SystemID) {
        unimplemented!()
    }
    pub fn select_object(&mut self, config: &mut Config, id: SystemID) -> Option<ObjectID> {
        let table: OptionTable = OptionTable::new("Please select an object".to_string(), self.sys.get_object_names_sys(id), config.context.grab(constants::SELECT));
        let temp = self.generic_select(config, &table, None, |x| x)?;
        Some(self.sys.get_system(id).get_objs()[temp])
    }
}
