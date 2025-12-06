use std::sync::{
    Arc,
    Mutex,
};

use anyhow::{
    Result,
    anyhow,
};
use godot::obj::Gd;
use iroh::endpoint::Connection;

use super::IrohConnection;
use crate::streams::{
    IrohRecvStream,
    IrohSendStream,
};

impl IrohConnection {
    pub(crate) async fn open_uni_logic(
        connection_ref: Arc<Mutex<Option<Connection>>>,
    ) -> Result<Gd<IrohSendStream>> {
        todo!();
    }

    // TODO: Figure out godot compatible return type containing both send and recv streams
    pub(crate) async fn open_bi_logic(
        connection_ref: Arc<Mutex<Option<Connection>>>,
    ) -> Result<()> {
        todo!();
    }

    pub(crate) async fn accept_uni_logic(
        connection_ref: Arc<Mutex<Option<Connection>>>,
    ) -> Result<Gd<IrohRecvStream>> {
        todo!();
    }

    // TODO: Figure out godot compatible return type containing both send and recv streams
    pub(crate) async fn accept_bi_logic(
        connection_ref: Arc<Mutex<Option<Connection>>>,
    ) -> Result<()> {
        todo!();
    }

    fn get_connection(connection_ref: Arc<Mutex<Option<Connection>>>) -> Result<Connection> {
        let connection_clone = {
            let guard = connection_ref.lock().unwrap();
            guard.clone()
        };

        let Some(connection) = connection_clone else {
            return Err(anyhow!("Connection is not yet initialized"));
        };

        Ok(connection)
    }
}
