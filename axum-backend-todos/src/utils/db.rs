use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};
use uuid::Uuid;

use crate::models::models_todos::Todo;

pub type Db = Arc<RwLock<HashMap<Uuid, Todo>>>;
