use crate::configuration_entry::configuration_entry::EMPTY_STRING;
use crate::configuration_entry::configuration_entry::FLAT_PATH_SEPARATOR;
use crate::configuration_entry::ConfigurationEntry;

impl ConfigurationEntry {
    pub fn remove_children_in_flat_path(&mut self, flat_path: &str) -> bool {
        let mut split_path: Vec<&str> = flat_path.split(FLAT_PATH_SEPARATOR).collect();
        if split_path.is_empty() {
            return false;
        }

        // root
        // - key
        // -- other_key
        // --- target_key - delete this
        // we have to execute for target_key that means getting flat path containing
        // only target_key flat path ie `key.other_key.target_key`
        // due to how execute_for_configuration_entry_in_flat_path works
        let maybe_last_key = split_path.pop();
        if maybe_last_key.is_none() {
            return false;
        }
        let last_key = maybe_last_key.unwrap();

        let maybe_pre_last_key = split_path.pop();
        let pre_last_key = maybe_pre_last_key.unwrap_or(EMPTY_STRING);

        if pre_last_key == EMPTY_STRING {
            return self.remove_child_by_key(last_key);
        }

        let remove_children_in_config_entry = |x: &mut ConfigurationEntry| {
            if pre_last_key == x.key && x.children.is_some() {
                x.remove_child_by_key(last_key);
            }
        };

        let flat_path_to_search_in = if !split_path.is_empty() {
            &split_path.join(FLAT_PATH_SEPARATOR)
        } else {
            pre_last_key
        };

        self.execute_for_configuration_entry_in_flat_path(flat_path_to_search_in, &remove_children_in_config_entry)
    }

    fn remove_child_by_key(&mut self, key: &str) -> bool {
        if self.children.is_none() {
            return false;
        }

        let mut children = self.children.clone().unwrap();
        let maybe_index = children.iter().position(|x| x.key == key);
        if maybe_index.is_none() {
            return false;
        }

        children.swap_remove(maybe_index.unwrap());
        if children.is_empty() {
            self.children = None
        } else {
            self.children = Some(children)
        }
        true
    }
}
