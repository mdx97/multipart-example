use std::str::FromStr;

use anyhow::anyhow;
use arma_rs::{Context, ContextState};
use uuid::Uuid;

use crate::multipart::SessionType;
use crate::result::{ArmaResult, IntoArmaError, IntoArmaResult};
use crate::{api, SessionState};

/// The session type for the `list_equipment` command.
#[derive(Clone)]
pub struct ListEquipment;

impl SessionType for ListEquipment {
    type Item = String;

    fn get_data() -> Vec<Self::Item> {
        api::list_equipment_classnames().unwrap_or_default()
    }
}

/// A command to return the whitelisted equipment from the API.
pub fn list_equipment(ctx: Context, session_id: String) -> ArmaResult {
    let session_id = Uuid::from_str(&session_id).ok();
    ctx.global()
        .get::<SessionState<ListEquipment>>()
        .ok_or_else(|| anyhow!("failed to get SessionManager instance for list_equipment"))
        .into_arma_error()?
        .to_owned()
        .lock()
        .unwrap()
        .get(session_id)
        .into_arma_result()
}
