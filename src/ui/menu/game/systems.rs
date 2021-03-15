use crate::{
    save::Package,
    systems::system_id::SystemID,
    ui::menu::{self, constants::SYSTEMS_CONTEXT},
};

use menu::options::OptionTable;
use menu::Config;
use menu::{grab_menu_res, MenuResult};
impl Package {
    pub fn systems_menu(&mut self, config: &mut Config) {
        loop {
            let options = OptionTable::new(String::new(), self.sys.display(), config.context.grab(SYSTEMS_CONTEXT));
            let res: MenuResult = grab_menu_res(&options, config, self);
            match res {
                MenuResult::Continue => continue,
                MenuResult::Exit => {
                    if self.confirm_esc() {
                        break;
                    } else {
                        continue;
                    }
                }
                MenuResult::Copy => self.copy_in_systems(),
                MenuResult::Paste => self.paste_in_systems(),
                MenuResult::Enter(val) => self.system_menu(config, SystemID::new(val)),
                MenuResult::New => self.new_in_systems(),
                MenuResult::Remove => self.remove_in_systems(),
            }
        }
    }
    pub fn paste_in_systems(&mut self) {
        unimplemented!()
    }
    pub fn copy_in_systems(&mut self) {
        unimplemented!()
    }
    pub fn confirm_esc(&mut self) -> bool {
        unimplemented!()
    }
    pub fn new_in_systems(&mut self) {
        unimplemented!()
    }
    pub fn remove_in_systems(&mut self) {
        unimplemented!()
    }
}
