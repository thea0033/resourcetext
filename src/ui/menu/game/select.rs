use crate::{
    save::Package,
    ui::menu::{config::Config, grab_menu_res, options::OptionTable},
};

impl Package {
    pub fn generic_select<T, C>(&mut self, config: &mut Config, table: &OptionTable, paste: Option<T>, convert: C) -> Option<T>
    where
        C: Fn(usize) -> T,
    {
        loop {
            match grab_menu_res(&table, config, self) {
                crate::ui::menu::MenuResult::Continue => {}
                crate::ui::menu::MenuResult::Exit => break None,
                crate::ui::menu::MenuResult::Paste => {
                    if let Some(val) = paste {
                        break Some(val);
                    } else {
                        self.illegal_state(config)
                    }
                }
                crate::ui::menu::MenuResult::Enter(val) => break Some(convert(val)),
                _ => self.illegal_state(config),
            }
        }
    }
}
