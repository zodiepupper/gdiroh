use std::sync::{
    Arc,
    Mutex,
};

use godot::prelude::*;
use iroh::endpoint::RecvStream;

#[derive(GodotClass)]
#[class(base=Object)]
pub struct IrohRecvStream {
    base: Base<Object>,
    pub(crate) recv_stream: Arc<Mutex<Option<RecvStream>>>,
}

#[godot_api]
impl IObject for IrohRecvStream {
    fn init(base: Base<Object>) -> Self {
        Self {
            base,
            recv_stream: Arc::new(Mutex::new(None)),
        }
    }
}
