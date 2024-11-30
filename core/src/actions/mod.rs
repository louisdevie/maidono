use crate::actions::group::GroupActions;
use crate::problem;
use crate::utils::{path, Location, Result};
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::{read_dir, DirEntry, File, ReadDir};
use std::path::Path;

mod action;
mod commands;
mod group;
mod host;
mod refs;

pub use action::Action;
pub use commands::Commands;
pub use group::Group;
pub use host::HostRef;

pub type AllGroupsResults = Result<Vec<(String, Result<Group>)>>;

pub struct AllGroups {
    groups: HashMap<String, Group>,
}

pub fn read_all_groups() -> Result<AllGroups> {
    let mut groups_map = HashMap::new();

    for (name, group_result) in try_read_all_groups()? {
        groups_map.insert(name, group_result?);
    }

    Ok(AllGroups { groups: groups_map })
}

pub fn try_read_all_groups() -> AllGroupsResults {
    let entries = list_action_files()?;
    let mut groups: Vec<(String, Result<Group>)> = Vec::new();
    let mut duplicates: HashSet<String> = HashSet::new();

    for (name, group_result) in
        entries.filter_map(|entry| entry.ok().and_then(read_group_from_entry))
    {
        if duplicates.contains(&name) {
            groups.push((name, Err(problem!("Duplicate group."))))
        } else {
            duplicates.insert(name.clone());
            groups.push((name, group_result))
        }
    }

    Ok(groups)
}

pub fn read_group_by_name(name: &'_ str) -> Result<Group> {
    let path = list_action_files()?
        .filter_map(|res| match res {
            Ok(entry) => Some(entry.path()),
            Err(_) => None,
        })
        .find(|path| {
            path.is_file()
                && match path.file_stem() {
                    Some(file_stem) => file_stem == name,
                    None => false,
                }
        })
        .ok_or(problem!("Group '{}' not found.", name))?;

    read_group(path)
}

fn read_group_from_entry(entry: DirEntry) -> Option<(String, Result<Group>)> {
    let path = entry.path();
    let name = path.file_stem()?.to_string_lossy().into_owned();

    Some((name, read_group(path)))
}

fn read_group<P: AsRef<Path>>(path: P) -> Result<Group> {
    let path_str = path.as_ref().as_os_str().to_string_lossy().into_owned();

    let file = File::open(path)?;

    let actions = serde_yaml::from_reader::<_, GroupActions>(file).map_err(|yaml_err| {
        let err = problem!("unable to parse file {}", path_str);

        let location = yaml_err
            .location()
            .map(|yaml_loc| Location::new(path_str, yaml_loc.line(), yaml_loc.column()));
        let cause = problem!(yaml_err);

        err.because(match location {
            Some(location) => cause.at(location),
            None => cause,
        })
    })?;

    Group::from_deserialized(actions)
}

fn list_action_files() -> Result<ReadDir> {
    read_dir(path::ACTIONS_CONFIG_DIR)
        .map_err(|err| problem!("Unable to read actions").because(err))
}

impl IntoIterator for AllGroups {
    type Item = (String, Group);

    type IntoIter = std::collections::hash_map::IntoIter<String, Group>;

    fn into_iter(self) -> Self::IntoIter {
        self.groups.into_iter()
    }
}
