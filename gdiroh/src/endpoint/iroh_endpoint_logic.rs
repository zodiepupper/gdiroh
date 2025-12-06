//! Defines the logic shared between [`IrohEndpoint`] functions.

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

use super::IrohEndpoint;
use crate::connection::IrohConnection;

impl IrohEndpoint {
    /// Internal shared async logic for:
    /// * [`IrohEndpoint::bind_blocking`]
    /// * [`IrohEndpoint::bind_async`]
    pub(crate) async fn bind_logic(
        endpoint_ref: Arc<Mutex<Option<Endpoint>>>,
        alpns: Vec<String>,
    ) -> Result<Endpoint> {
        if endpoint_ref.lock().unwrap().is_some() {
            godot_warn!("Endpoint is not empty, overwriting...");
        }

        let alpn_byte_arrays: Vec<Vec<u8>> =
            alpns.iter().map(|alpn| alpn.clone().into_bytes()).collect();

        Endpoint::builder()
            .alpns(alpn_byte_arrays)
            .bind()
            .await
            .map_err(|err| anyhow!("{err}"))
    }

    // TODO: Make `endpoint_address` a static type

    /// Internal shared async logic for:
    /// * [`IrohEndpoint::connect_blocking`]
    /// * [`IrohEndpoint::connect_async`]
    pub(crate) async fn connect_logic(
        endpoint_ref: Arc<Mutex<Option<Endpoint>>>,
        endpoint_address: String,
        alpn: String,
    ) -> Result<Gd<IrohConnection>> {
        let endpoint = Self::get_endpoint(endpoint_ref)?;

        let endpoint_pub_key = endpoint_address
            .to_string()
            .parse::<iroh::PublicKey>()
            .context(format!(
                "\"{endpoint_address}\" is not parsable as a PublicKey"
            ))?;

        let alpn = alpn.into_bytes();

        let connection = endpoint.connect(endpoint_pub_key, &alpn).await?;

        let mut iroh_connection = IrohConnection::new_alloc();
        *iroh_connection.bind_mut().connection.lock().unwrap() = Some(connection);

        Ok(iroh_connection)
    }

    /// Internal shared async logic for:
    /// * [`IrohEndpoint::accept_blocking`]
    /// * [`IrohEndpoint::accept_async`]
    pub(crate) async fn accept_logic(
        endpoint_ref: Arc<Mutex<Option<Endpoint>>>,
    ) -> Result<Gd<IrohConnection>> {
        let endpoint = Self::get_endpoint(endpoint_ref)?;

        let Some(incoming) = endpoint.accept().await else {
            return Err(anyhow!("endpoint.accept() returned None"));
        };

        let connection = incoming.await?;

        let mut iroh_connection = IrohConnection::new_alloc();
        *iroh_connection.bind_mut().connection.lock().unwrap() = Some(connection);

        Ok(iroh_connection)
    }

    fn get_endpoint(endpoint_ref: Arc<Mutex<Option<Endpoint>>>) -> Result<Endpoint> {
        let endpoint_clone = {
            let guard = endpoint_ref.lock().unwrap();
            guard.clone()
        };

        match endpoint_clone {
            Some(endpoint) => {
                return Ok(endpoint);
            }
            None => {
                return Err(anyhow!("Endpoint is not yet initialized"));
            }
        }
    }
}
