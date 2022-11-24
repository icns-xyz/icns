#![cfg(test)]

use super::helpers::instantiate_with_name_nft_and_admins;

#[test]
fn only_admin_can_set_record() {
    let admin1 = String::from("admin1");
    let admin2 = String::from("admin2");
    let admins = vec![admin1.clone(), admin2.clone()];
}