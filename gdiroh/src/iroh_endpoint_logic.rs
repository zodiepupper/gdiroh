use std::sync::{
    Arc,
    Mutex,
};

use anyhow::{
    Context,
    Result,
    anyhow,
};
use godot::prelude::*;
use iroh::Endpoint;

use crate::{
    iroh_connection::IrohConnection,
    iroh_endpoint::IrohEndpoint,
};

impl IrohEndpoint {
    pub(crate) async fn bind_logic(
        endpoint_ref: Arc<Mutex<Option<Endpoint>>>,
        alpn: String,
    ) -> Result<Endpoint> {
        godot_print!("test1");

        if endpoint_ref.lock().unwrap().is_some() {
            godot_warn!("Endpoint is not empty, overwriting...");
        }

        let alpn = alpn.into_bytes();

        Endpoint::builder()
            .alpns(vec![alpn])
            .bind()
            .await
            .map_err(|err| anyhow!("{err}"))
    }

    pub(crate) async fn connect_logic(
        endpoint_ref: Arc<Mutex<Option<Endpoint>>>,
        endpoint_address: String,
        alpn: String,
    ) -> Result<Gd<IrohConnection>> {
        let endpoint_clone = {
            let guard = endpoint_ref.lock().unwrap();
            guard.clone()
        };

        let Some(endpoint) = endpoint_clone else {
            return Err(anyhow!("Endpoint is not yet initialized"));
        };

        let endpoint_pub_key = endpoint_address
            .to_string()
            .parse::<iroh::PublicKey>()
            .context(format!(
                "\"{endpoint_address}\" is not parsable as a PublicKey"
            ))?;

        let alpn = alpn.into_bytes();

        let connection = endpoint
            .connect(endpoint_pub_key, &alpn)
            .await
            .context("Connection failed")?;

        let mut iroh_connection = IrohConnection::new_alloc();
        *iroh_connection.bind_mut().connection.lock().unwrap() = Some(connection);

        Ok(iroh_connection)
    }

    pub(crate) async fn accept_logic(
        endpoint_ref: Arc<Mutex<Option<Endpoint>>>,
    ) -> Result<Gd<IrohConnection>> {
        let endpoint_clone = {
            let guard = endpoint_ref.lock().unwrap();
            guard.clone()
        };

        let Some(endpoint) = endpoint_clone else {
            return Err(anyhow!("Endpoint is not yet initialized"));
        };

        let Some(incoming) = endpoint.accept().await else {
            return Err(anyhow!("endpoint.accept() returned None"));
        };

        let connection = incoming.await.context("Connection failed")?;

        let mut iroh_connection = IrohConnection::new_alloc();
        *iroh_connection.bind_mut().connection.lock().unwrap() = Some(connection);

        Ok(iroh_connection)
    }
}
