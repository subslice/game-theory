pub use crate::{
    impls::lifecycle::{ data, lifecycle },
    traits::lifecycle::Lifecycle
};
use ink::primitives::AccountId;
use openbrush::traits::Storage;

// impl<T: Storage<data::Data>> Lifecycle for T {
//     default fn get_players(&self) -> Vec<AccountId> {
//         self.data::<data::Data>().players.clone()
//     }
// }
