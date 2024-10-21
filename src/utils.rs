use crate::*;

pub fn account_vec_to_set(
    account_vec: Vec<AccountId>,
    storage_key: &'static [u8],
) -> IterableSet<AccountId> {
    let mut set = IterableSet::new(storage_key);
    for element in account_vec.iter() {
        set.insert(element.clone());
    }
    set
}