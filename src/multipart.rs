use std::collections::HashMap;

use anyhow::{anyhow, Result};
use arma_rs::{IntoArma, Value};
use uuid::Uuid;

pub const PAGE_SIZE: usize = 100;

/// A type that manages the progress of multipart responses.
pub struct Sessions<T: SessionType> {
    sessions: HashMap<Uuid, Vec<T::Item>>,
}

impl<T> Sessions<T>
where
    T: Clone + SessionType,
{
    pub fn new() -> Self {
        Self {
            sessions: HashMap::default(),
        }
    }

    /// Get the next message part in the sequence, optionally for a given session ID.
    ///
    /// If no session ID is provided, a new session is started and the returned session ID should
    /// be used in the next request.
    pub fn get(&mut self, session_id: Option<Uuid>) -> Result<MessagePart<T::Item>> {
        let values = if let Some(session_id) = session_id {
            self.sessions
                .get(&session_id)
                .ok_or_else(|| anyhow!("session with given id does not exist"))?
                .to_owned()
        } else {
            T::get_data()
        };

        // TEMP: We should ideally calculate the number of values that we can return at once on the
        // fly. Using a fixed PAGE_SIZE is a temporary hack.
        let (values, left) = if values.len() > PAGE_SIZE {
            let split = values.split_at(PAGE_SIZE);
            (Vec::from(split.0), Vec::from(split.1))
        } else {
            (values, vec![])
        };

        let session_existed = session_id.is_some();
        let session_id = session_id.unwrap_or(Uuid::new_v4());

        let has_more = if left.is_empty() {
            if session_existed {
                self.sessions.remove(&session_id);
            }
            false
        } else {
            self.sessions.insert(session_id, left);
            true
        };

        Ok(MessagePart {
            values,
            session_id,
            has_more,
        })
    }
}

/// A trait that defines what data is returned for different commands that utilize multipart responses.
pub trait SessionType {
    type Item: Clone;

    fn get_data() -> Vec<Self::Item>;
}

/// A type that represents the data to be returned to ArmA per message part request.
pub struct MessagePart<T> {
    pub values: Vec<T>,
    pub session_id: Uuid,
    pub has_more: bool,
}

impl<T> IntoArma for MessagePart<T>
where
    T: IntoArma,
{
    fn to_arma(&self) -> Value {
        Value::Array(vec![
            self.values.to_arma(),
            Value::String(self.session_id.to_string()),
            Value::Boolean(self.has_more),
        ])
    }
}
