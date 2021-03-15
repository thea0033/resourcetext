use crate::{
    save::Package,
    systems::object_id::ObjectID,
    ui::menu::{constants::OBJECT_CONTEXT, grab_menu_res, options::OptionTable, Config, MenuResult},
};

impl Package {
    pub const OBJ_OPTIONS: &'static [&'static str] = &["Enter instructions menu", "Perform a recipe", "Transfer resources to another object"];
    pub fn object_menu(&mut self, config: &mut Config, id: ObjectID) {
        loop {
            let options: String = self.sys.get_object(id).display_extras(&self.rss, &self.cmp);
            let table = OptionTable::new(options, Package::generate_object_options(), config.context.grab(OBJECT_CONTEXT));
            let res: MenuResult = grab_menu_res(&table, config, self);
            match res {
                MenuResult::Continue => continue,
                MenuResult::Exit => break,
                MenuResult::Copy => self.copy_in_object(id),
                MenuResult::Paste => self.paste_in_object(id),
                MenuResult::Enter(0) => {}
                MenuResult::Enter(1) => {}
                MenuResult::Enter(2) => {}
                MenuResult::Enter(_) => {
                    println!("This shouldn't happen!");
                }
                MenuResult::New => self.new_in_object(id),
                MenuResult::Remove => self.remove_in_object(id),
            }
        }
    }
    pub fn generate_object_options() -> Vec<String> {
        Package::OBJ_OPTIONS.to_vec().iter().map(|x| x.to_string()).collect()
    }
    pub fn copy_in_object(&mut self, id: ObjectID) {
        unimplemented!()
    }
    pub fn paste_in_object(&mut self, id: ObjectID) {
        unimplemented!()
    }
    pub fn new_in_object(&mut self, id: ObjectID) {
        unimplemented!()
    }
    pub fn remove_in_object(&mut self, id: ObjectID) {
        unimplemented!()
    }
    pub fn transfer(&mut self, id: ObjectID) {}
}
