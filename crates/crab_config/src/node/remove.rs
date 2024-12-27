use crate::node::{ConfigurationNode, FLAT_PATH_SEPARATOR, NodeContent};

impl ConfigurationNode {
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
        // due to how execute_for_configuration_node_in_flat_path works
        let Some(last_key) = split_path.pop() else {
            return false;
        };

        let Some(pre_last_key) = split_path.pop() else {
            return self.remove_child_by_key(last_key);
        };

        let remove_children_in_config_entry = |x: &mut ConfigurationNode| {
            x.remove_child_by_key(last_key);
        };

        let flat_path_to_search_in = if !split_path.is_empty() {
            &split_path.join(FLAT_PATH_SEPARATOR)
        } else {
            pre_last_key
        };

        self.execute_for_configuration_node_in_flat_path(flat_path_to_search_in, &remove_children_in_config_entry)
    }

    fn remove_child_by_key(&mut self, key: &str) -> bool {
        if let NodeContent::Children(children) = &mut self.content {
            children.retain(|(child_key, _)| child_key != key);
            true
        } else {
            false
        }
    }
}
