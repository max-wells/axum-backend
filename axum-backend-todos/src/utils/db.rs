use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use crate::models::models_todos::Todo;

pub type DbArcRwLock = Arc<RwLock<HashMap<i32, Todo>>>;
