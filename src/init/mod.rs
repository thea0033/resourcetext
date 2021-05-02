use crate::{
    component::readable::ReadableComponentDict,
    constants::*,
    file::read_basic,
    merge::Merge,
    resources::readable::ReadableResourceDict,
    save::{readable::ReadablePackage, Package},
    systems::readable::ReadableSystems,
};

use crate::instr::directions::Directions;

fn generate_package_orig(path: &str) -> Result<ReadablePackage, serde_json::Error> {
    let rss = rss(path)?;
    let cmp = cmp(path)?;
    let sys = sys(path)?;
    Ok(ReadablePackage::new(rss, cmp, sys))
}
pub fn generate_package(path: Vec<&str>) -> Result<Package, String> {
    let mut iter = path.into_iter();
    let first = iter.next().ok_or_else(|| "There were no paths!".to_string())?;
    let mut final_package = generate_package_orig(first).map_err(|x| x.to_string())?;
    for i in iter {
        final_package.merge(generate_package_orig(i).map_err(|x| x.to_string())?);
    }
    convert_package(final_package)
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
fn sys(path: &str) -> Result<ReadableSystems, serde_json::Error> {
    let file = format!("{}{}", path, SYSTEMS);
    serde_json::from_str(&read_basic(&file))
}
fn rss(path: &str) -> Result<ReadableResourceDict, serde_json::Error> {
    let file = format!("{}{}", path, RESOURCES);
    serde_json::from_str(&read_basic(&file))
}
fn cmp(path: &str) -> Result<ReadableComponentDict, serde_json::Error> {
    let file = format!("{}{}", path, COMPONENTS);
    serde_json::from_str(&read_basic(&file))
}
