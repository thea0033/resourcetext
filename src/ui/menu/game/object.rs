use crate::{
    save::Package,
    systems::{object_id::ObjectID, system_id::SystemID},
    ui::menu::{constants::OBJECT, grab_menu_res, options::OptionTable, Config, MenuResult},
};

impl Package {
    pub const OBJ_OPTIONS: &'static [&'static str] = &["Enter instructions menu", "Perform a recipe", "Transfer resources to another object"];
    pub fn object_menu(&mut self, config: &mut Config, id: ObjectID) {
        loop {
            let options: String = self.sys.get_object(id).display_extras(&self.rss, &self.cmp);
            let table = OptionTable::new(options, Package::generate_object_options(), config.context.grab(OBJECT));
            let res: MenuResult = grab_menu_res(&table, config, self);
            match res {
                MenuResult::Continue => continue,
                MenuResult::Exit => break,
                MenuResult::Copy => self.copy_in_object(id),
                MenuResult::Paste => self.paste_in_object(id),
                MenuResult::Enter(0) => self.instrs_menu(config, id),
                MenuResult::Enter(1) => self.perform_recipe(config, id),
                MenuResult::Enter(2) => {}
                MenuResult::Enter(_) => println!("This shouldn't happen!"),
                MenuResult::New => self.new_in_object(config, id),
                MenuResult::Remove => self.remove_in_object(config, id),
            }
        }
    }
    pub fn generate_object_options() -> Vec<String> {
        Package::OBJ_OPTIONS.to_vec().iter().map(|x| x.to_string()).collect()
    }
    fn copy_in_object(&mut self, id: ObjectID) {
        unimplemented!()
    }
    fn paste_in_object(&mut self, id: ObjectID) {
        unimplemented!()
    }
    fn new_in_object(&mut self, config: &mut Config, id: ObjectID) {
        self.install_components(config, id);
    }
    fn remove_in_object(&mut self, config: &mut Config, id: ObjectID) {
        self.remove_components(config, id);
    }
    pub fn transfer(&mut self, id: ObjectID) {}
}
