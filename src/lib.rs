mod api;
mod command;
mod multipart;
mod result;

use std::sync::{Arc, Mutex};

use arma_rs::{arma, Extension};

use crate::command::list_equipment::*;
use crate::multipart::Sessions;

pub type SessionState<T> = Arc<Mutex<Sessions<T>>>;

#[arma]
fn init() -> Extension {
    Extension::build()
        .command("list_equipment", list_equipment)
        .state::<SessionState<ListEquipment>>(Arc::new(Mutex::new(Sessions::new())))
        .finish()
}
