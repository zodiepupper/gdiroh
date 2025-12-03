//! TODO: Implement

use std::sync::{
    Arc,
    Mutex,
};

use godot::prelude::*;
use iroh::endpoint::Connection;

#[derive(GodotClass)]
#[class(base=Object)]
pub struct IrohConnection {
    base: Base<Object>,
    // TODO: This may not need to be a mutex
    pub(crate) connection: Arc<Mutex<Option<Connection>>>,
}

#[godot_api]
impl IObject for IrohConnection {
    fn init(base: Base<Object>) -> Self {
        Self {
            base,
            connection: Arc::new(Mutex::new(None)),
        }
    }
}
