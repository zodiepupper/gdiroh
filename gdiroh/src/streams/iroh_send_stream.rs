use std::sync::{
    Arc,
    Mutex,
};

use godot::prelude::*;
use iroh::endpoint::SendStream;

#[derive(GodotClass)]
#[class(base=Object)]
pub struct IrohSendStream {
    base: Base<Object>,
    pub(crate) send_stream: Arc<Mutex<Option<SendStream>>>,
}

#[godot_api]
impl IObject for IrohSendStream {
    fn init(base: Base<Object>) -> Self {
        Self {
            base,
            send_stream: Arc::new(Mutex::new(None)),
        }
    }
}
