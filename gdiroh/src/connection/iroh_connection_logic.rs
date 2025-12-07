use std::sync::{
    Arc,
    Mutex,
};

use anyhow::{
    Result,
    anyhow,
};
use godot::obj::{
    Gd,
    NewAlloc,
};
use iroh::endpoint::Connection;

use super::IrohConnection;
use crate::streams::IrohStream;

impl IrohConnection {
    pub(crate) async fn open_uni_logic(
        connection_ref: Arc<Mutex<Option<Connection>>>,
    ) -> Result<Gd<IrohStream>> {
        let connection = Self::get_connection(connection_ref)?;

        let send_stream = connection.open_uni().await?;

        let mut iroh_stream = IrohStream::new_alloc();
        *iroh_stream.bind_mut().send_stream.lock().unwrap() = Some(send_stream);

        Ok(iroh_stream)
    }

    pub(crate) async fn open_bi_logic(
        connection_ref: Arc<Mutex<Option<Connection>>>,
    ) -> Result<Gd<IrohStream>> {
        let connection = Self::get_connection(connection_ref)?;

        let (send_stream, recv_stream) = connection.open_bi().await?;

        let mut iroh_stream = IrohStream::new_alloc();
        *iroh_stream.bind_mut().recv_stream.lock().unwrap() = Some(recv_stream);
        *iroh_stream.bind_mut().send_stream.lock().unwrap() = Some(send_stream);

        Ok(iroh_stream)
    }

    pub(crate) async fn accept_uni_logic(
        connection_ref: Arc<Mutex<Option<Connection>>>,
    ) -> Result<Gd<IrohStream>> {
        let connection = Self::get_connection(connection_ref)?;

        let recv_stream = connection.accept_uni().await?;

        let mut iroh_stream = IrohStream::new_alloc();
        *iroh_stream.bind_mut().recv_stream.lock().unwrap() = Some(recv_stream);

        Ok(iroh_stream)
    }

    pub(crate) async fn accept_bi_logic(
        connection_ref: Arc<Mutex<Option<Connection>>>,
    ) -> Result<Gd<IrohStream>> {
        let connection = Self::get_connection(connection_ref)?;

        let (send_stream, recv_stream) = connection.accept_bi().await?;

        let mut iroh_stream = IrohStream::new_alloc();
        *iroh_stream.bind_mut().recv_stream.lock().unwrap() = Some(recv_stream);
        *iroh_stream.bind_mut().send_stream.lock().unwrap() = Some(send_stream);

        Ok(iroh_stream)
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
