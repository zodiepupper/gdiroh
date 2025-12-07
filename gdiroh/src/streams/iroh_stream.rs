use std::sync::{
    Arc,
    Mutex,
};

use godot::prelude::*;
use iroh::endpoint::{
    RecvStream,
    SendStream,
};

#[derive(GodotClass)]
#[class(base=Object)]
pub struct IrohStream {
    base: Base<Object>,
    pub(crate) recv_stream: Arc<Mutex<Option<RecvStream>>>,
    pub(crate) send_stream: Arc<Mutex<Option<SendStream>>>,
}

#[godot_api]
impl IObject for IrohStream {
    fn init(base: Base<Object>) -> Self {
        Self {
            base,
            recv_stream: Arc::new(Mutex::new(None)),
            send_stream: Arc::new(Mutex::new(None)),
        }
    }
}
