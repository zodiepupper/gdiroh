//! TODO: Implement

use std::sync::{
    Arc,
    Mutex,
};

use godot::prelude::*;
use iroh::endpoint::Connection;

use crate::streams::{
    IrohRecvStream,
    IrohSendStream,
};

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

#[godot_api]
impl IrohConnection {
    // #region open_uni

    #[func]
    fn open_uni_blocking(&self) -> Option<Gd<IrohSendStream>> {
        todo!()
    }

    #[func]
    fn open_uni_async(&self) -> Option<Gd<IrohSendStream>> {
        todo!()
    }

    #[signal]
    fn open_uni_async_results(result: Option<Gd<IrohSendStream>>);

    // #endregion

    // #region open_bi

    // TODO: Figure out godot compatible return type containing both send and recv streams

    #[func]
    fn open_bi_blocking(&self) {
        todo!()
    }

    #[func]
    fn open_bi_async(&self) {
        todo!()
    }

    #[signal]
    fn open_bi_async_results(result: ());

    // #endregion

    // #region accept_uni

    #[func]
    fn accept_uni_blocking(&self) -> Option<Gd<IrohRecvStream>> {
        todo!()
    }

    #[func]
    fn accept_uni_async(&self) -> Option<Gd<IrohRecvStream>> {
        todo!()
    }

    #[signal]
    fn accept_uni_async_results(result: Option<Gd<IrohRecvStream>>);

    // #endregion

    // #region accept_bi

    // TODO: Figure out godot compatible return type containing both send and recv streams

    #[func]
    fn accept_bi_blocking(&self) {
        todo!()
    }

    #[func]
    fn accept_bi_async(&self) {
        todo!()
    }

    #[signal]
    fn accept_bi_async_results(result: ());

    // #endregion
}
