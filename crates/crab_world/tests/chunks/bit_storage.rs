use crab_world::chunks::bit_storage::BitStorage;

fn get_test_values() -> (BitStorage, Vec<i64>) {
    let numbers: Vec<i64> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    (BitStorage::from_iter(numbers.iter().cloned(), 10), numbers)
}

#[test]
fn set() {
    let (mut bit_storage, mut numbers) = get_test_values();

    bit_storage.set(0, 0).unwrap();
    numbers[0] = 0;
    bit_storage.set(5, 3).unwrap();
    numbers[5] = 3;

    assert!(bit_storage.iter().eq(numbers.iter().cloned()));
}

#[test]
fn get() {
    let (bit_storage, numbers) = get_test_values();

    for (index, number) in numbers.iter().enumerate() {
        assert_eq!(bit_storage.get(index).unwrap(), *number);
    }
}

#[test]
fn iter() {
    let (bit_storage, numbers) = get_test_values();

    assert!(bit_storage.iter().eq(numbers.iter().cloned()));
}

#[test]
fn resize() {
    let (bit_storage, _numbers) = get_test_values();

    let resized = bit_storage.resized(5);

    assert!(bit_storage.iter().eq(resized.iter()));
}
