use std::sync::{
    Arc,
    Mutex,
};

use godot::prelude::*;
use iroh::Endpoint;

use crate::{
    helpers::create_signal_emitter,
    iroh_connection::IrohConnection,
    runtime::IrohRuntime,
};

#[derive(GodotClass)]
#[class(base=Object)]
/// Controls an iroh endpoint, establishing connections with other endpoints.
pub struct IrohEndpoint {
    base: Base<Object>,
    pub(crate) endpoint: Arc<Mutex<Option<Endpoint>>>,
}

#[godot_api]
impl IObject for IrohEndpoint {
    /// Note: This class defaults with empty internal data.
    /// Use `bind_blocking()` or `bind_async()` on a constructed instance to initialize it's endpoint.
    fn init(base: Base<Object>) -> Self {
        Self {
            base,
            endpoint: Arc::new(Mutex::new(None)),
        }
    }
}

#[godot_api]
impl IrohEndpoint {
    // #region bind

    #[func]
    /// Initializes the endpoint with default settings.
    ///
    /// # Arguments
    ///
    /// * `alpn` - The ALPN protocol this endpoint will accept on incoming connections.
    ///
    /// # Returns
    ///
    /// `true` when the endpoint was successfully written, `false` otherwise.
    fn bind_blocking(&self, alpn: GString) -> bool {
        let endpoint_ref = self.endpoint.clone();
        let alpn = alpn.to_string();

        let result = IrohRuntime::block_on(Self::bind_logic(endpoint_ref, alpn));

        match result {
            Ok(endpoint) => {
                *self.endpoint.lock().unwrap() = Some(endpoint);
                return true;
            }
            Err(err) => {
                godot_error!("Failed to bind endpoint: {err}");
                return false;
            }
        }
    }

    #[func]
    /// Initializes the endpoint with default settings asynchronously.
    ///
    /// # Arguments
    ///
    /// * `alpn` - The ALPN protocol this endpoint will accept on incoming connections.
    ///
    /// # Returns
    ///
    /// Void. Use `bind_async_result` to await results.
    fn bind_async(&self, alpn: GString) {
        let instance_id = self.base().instance_id();
        let endpoint_ref = self.endpoint.clone();
        let alpn = alpn.to_string();

        IrohRuntime::spawn(async move {
            let push_result = create_signal_emitter::<Self, bool>(instance_id, "bind_async_result");

            let result = Self::bind_logic(endpoint_ref.clone(), alpn).await;

            match result {
                Ok(endpoint) => {
                    *endpoint_ref.lock().unwrap() = Some(endpoint);
                    push_result(true);
                    return;
                }
                Err(err) => {
                    godot_error!("Failed to bind endpoint: {err}");
                    push_result(false);
                    return;
                }
            }
        });
    }

    #[signal]
    /// Provides results from `bind_async()` function calls.
    ///
    /// # Returns
    ///
    /// * `result: bool` - `true` when the endpoint was successfully written, `false` otherwise.
    fn bind_async_result(result: bool);

    // #endregion

    // #region connect

    #[func]
    /// Creates a connection to a remote `IrohEndpoint`.
    ///
    /// # Arguments
    ///
    /// * `endpoint_address` - The public key (address) of the target peer.
    /// * `alpn` - The ALPN protocol to use. The remote endpoint must support this alpn.
    ///
    /// # Returns
    ///
    /// `null` if an error was encountered, otherwise a `IrohConnection` containing the negotiated connection.
    fn connect_blocking(
        &self,
        endpoint_address: GString,
        alpn: GString,
    ) -> Option<Gd<IrohConnection>> {
        let endpoint_ref = self.endpoint.clone();
        let endpoint_address = endpoint_address.to_string();
        let alpn = alpn.to_string();

        let result =
            IrohRuntime::block_on(Self::connect_logic(endpoint_ref, endpoint_address, alpn));

        match result {
            Ok(iroh_connection) => {
                return Some(iroh_connection);
            }
            Err(err) => {
                godot_error!("Connection failed: {err}");
                return None;
            }
        };
    }

    #[func]
    /// Creates a connection to a remote `IrohEndpoint` asynchronously.
    ///
    /// # Arguments
    ///
    /// * `endpoint_address` - The public key (address) of the target peer.
    /// * `alpn` - The ALPN protocol to use. The remote endpoint must support this alpn.
    ///
    /// # Returns
    ///
    /// Void. Use `connect_async_result` to await results.
    fn connect_async(&self, endpoint_address: GString, alpn: GString) {
        let instance_id = self.base().instance_id();
        let endpoint_ref = self.endpoint.clone();
        let endpoint_address = endpoint_address.to_string();
        let alpn = alpn.to_string();

        IrohRuntime::spawn(async move {
            let push_result = create_signal_emitter::<Self, Option<Gd<IrohConnection>>>(
                instance_id,
                "connect_async_result",
            );

            let result = Self::connect_logic(endpoint_ref, endpoint_address, alpn).await;

            match result {
                Ok(iroh_connection) => {
                    push_result(Some(iroh_connection));
                    return;
                }
                Err(err) => {
                    godot_error!("Connection failed: {err}");
                    return;
                }
            };
        });
    }

    #[signal]
    /// Provides results from `connect_async()` function calls.
    ///
    /// # Returns
    ///
    /// * `result: IrohConnection` - `null` if an error was encountered, otherwise a `IrohConnection` containing the negotiated connection.
    fn connect_async_result(result: Option<Gd<IrohConnection>>);

    // #endregion

    // #region accept

    #[func]
    /// Waits for and accepts an incoming connection on the endpoint.
    ///
    /// # Returns
    ///
    /// `null` if an error was encountered, otherwise a `IrohConnection` containing the negotiated connection.
    fn accept_blocking(&self) -> Option<Gd<IrohConnection>> {
        let endpoint_ref = self.endpoint.clone();

        let result = IrohRuntime::block_on(Self::accept_logic(endpoint_ref));

        match result {
            Ok(iroh_connection) => {
                return Some(iroh_connection);
            }
            Err(err) => {
                godot_error!("Connection failed: {err}");
                return None;
            }
        };
    }

    #[func]
    /// Asynchronously waits for and accepts an incoming connection on the endpoint.
    ///
    /// # Returns
    ///
    /// Void. Use `accept_async_result` to await results.
    fn accept_async(&self) {
        let instance_id = self.base().instance_id();
        let endpoint_ref = self.endpoint.clone();

        IrohRuntime::spawn(async move {
            let push_result = create_signal_emitter::<Self, Option<Gd<IrohConnection>>>(
                instance_id,
                "accept_async_result",
            );

            let result = Self::accept_logic(endpoint_ref).await;

            match result {
                Ok(iroh_connection) => {
                    push_result(Some(iroh_connection));
                    return;
                }
                Err(err) => {
                    godot_error!("Connection failed: {err}");
                    return;
                }
            };
        });
    }

    #[signal]
    /// Provides results from `connect_async()` function calls.
    ///
    /// # Returns
    ///
    /// * `result: IrohConnection` - `null` if an error was encountered, otherwise a `IrohConnection` containing the negotiated connection.
    fn accept_async_result(result: Option<Gd<IrohConnection>>);

    // #endregion

    #[func]
    /// Returns the current address (public key) of this endpoint as a string.
    /// If the endpoint does not yet exist "N/A" is returned instead.

    // Note: Option<GString> makes gdext cry, so we just use a default string instead of null.
    fn address(&self) -> GString {
        let Some(ref endpoint) = *self.endpoint.lock().unwrap() else {
            godot_error!("Endpoint is not initialized yet");
            return GString::from("N/A");
        };

        let key = endpoint.addr().id.to_string();
        return GString::from(key.as_str());
    }
}
