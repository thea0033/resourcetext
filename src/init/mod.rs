use std::{sync::mpsc::{Sender, channel}, thread::spawn};

use crate::{component::readable::ReadableComponentDict, constants::*, file::read_basic, merge::Merge, resources::readable::ReadableResourceDict, save::{readable::ReadablePackage, Package}, systems::readable::ReadableSystems, ui::menu::graphics::loading_screen};

use crate::instr::directions::Directions;

fn generate_package_orig(path: String, send: &mut Sender<Option<String>>, i: usize) -> Result<ReadablePackage, serde_json::Error> {
    send.send(Some("Compiling resources for package ".to_string() + &i.to_string() + "...")).expect("FAILURE");
    let rss = rss(path.clone())?;
    send.send(Some("Compiling components for package ".to_string() + &i.to_string() + "...")).expect("FAILURE");
    let cmp = cmp(path.clone())?;
    send.send(Some("Compiling systems for package ".to_string() + &i.to_string() + "...")).expect("FAILURE");
    let sys = sys(path)?;
    Ok(ReadablePackage::new(rss, cmp, sys))
}
pub fn generate_package(path: Vec<String>) -> Result<Package, String> {
    let (mut send, recv) = channel();
    let temp = (path.len() * 4) as u64;
    let handle = spawn(move || loading_screen(recv, temp, 500, 20));
    let iter = path.into_iter().enumerate();
    let mut final_package = ReadablePackage::default();
    for (i, item) in iter {
        let temp = generate_package_orig(item, &mut send, i).map_err(|x| x.to_string())?;
        send.send(Some("Merging package ".to_string() + &i.to_string() + " to final...")).expect("FAILURE");
        final_package.merge(temp);
    }
    let temp = convert_package(final_package);
    send.send(Some("Converting final package...".to_string())).expect("FAILURE");
    handle.join().expect("FAILED");
    temp
}
fn convert_package(orig: ReadablePackage) -> Result<Package, String> {
    let rss = orig
        .rss
        .to_usable()
        .ok_or_else(|| "Failure converting resources from readable to usable".to_string())?;
    let cmp = orig
        .cmp
        .convert(&rss)
        .ok_or_else(|| "Failure converting components from readable to usable".to_string())?;
    let mut dir = Directions::new();
    let sys = orig.sys.convert(&rss, &cmp, &mut dir)?;
    Ok(Package::new(rss, cmp, sys, dir))
}
fn sys(path: String) -> Result<ReadableSystems, serde_json::Error> {
    let file = format!("{}{}", path, SYSTEMS);
    serde_json::from_str(&read_basic(&file))
}
fn rss(path: String) -> Result<ReadableResourceDict, serde_json::Error> {
    let file = format!("{}{}", path, RESOURCES);
    serde_json::from_str(&read_basic(&file))
}
fn cmp(path: String) -> Result<ReadableComponentDict, serde_json::Error> {
    let file = format!("{}{}", path, COMPONENTS);
    serde_json::from_str(&read_basic(&file))
}
