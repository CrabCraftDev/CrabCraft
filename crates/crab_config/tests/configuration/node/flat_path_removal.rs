use crate::utils::get_test_configuration_node;

#[test]
fn flat_path_removal() {
    let mut test_config_entry = get_test_configuration_node();
    let initial_map = test_config_entry
        .serialize_to_flatten_map_with_path()
        .expect("Failed to serialize initial configuration");
    let initial_keys: Vec<String> = initial_map.keys().cloned().collect();

    const PATH_TO_REMOVE: &str = "1234.12345";
    let removal_success = test_config_entry.remove_children_in_flat_path(PATH_TO_REMOVE);

    assert!(removal_success, "Failed to remove children at path: {}", PATH_TO_REMOVE);

    let final_map = test_config_entry
        .serialize_to_flatten_map_with_path()
        .expect("Failed to serialize final configuration");
    let final_keys: Vec<String> = final_map.keys().cloned().collect();

    assert!(
        !final_keys.iter().any(|k| k.starts_with(PATH_TO_REMOVE)),
        "Path {} or its children still exist after removal",
        PATH_TO_REMOVE
    );

    for key in initial_keys.iter() {
        if !key.starts_with(PATH_TO_REMOVE) {
            assert!(
                final_keys.contains(key),
                "Unrelated path {} was unexpectedly removed",
                key
            );
        }
    }
}
