use crate::{
    save::Package,
    systems::system_id::SystemID,
    ui::menu::{
        self,
        constants::{self, SYSTEMS},
    },
};

use menu::options::OptionTable;
use menu::Config;
use menu::{grab_menu_res, MenuResult};
impl Package {
    pub fn systems_menu(&mut self, config: &mut Config) {
        loop {
            let options = OptionTable::new(String::new(), self.sys.display(), config.context.grab(SYSTEMS));
            let res: MenuResult = grab_menu_res(&options, config, self);
            match res {
                MenuResult::Continue => continue,
                MenuResult::Exit => break,
                MenuResult::Copy => self.copy_in_systems(),
                MenuResult::Paste => self.paste_in_systems(),
                MenuResult::Enter(val) => self.system_menu(config, SystemID::new(val)),
                MenuResult::New => self.new_in_systems(),
                MenuResult::Remove => self.remove_in_systems(),
            }
        }
    }
    fn paste_in_systems(&mut self) {
        unimplemented!()
    }
    fn copy_in_systems(&mut self) {
        unimplemented!()
    }
    fn new_in_systems(&mut self) {
        unimplemented!()
    }
    fn remove_in_systems(&mut self) {
        unimplemented!()
    }
    pub fn select_system(&mut self, config: &mut Config) -> Option<SystemID> {
        let table: OptionTable = OptionTable::new("Select a system:".to_string(), self.sys.display(), config.context.grab(constants::SELECT));
        self.generic_select(config, &table, None, |x| SystemID::new(x))
    }
}
