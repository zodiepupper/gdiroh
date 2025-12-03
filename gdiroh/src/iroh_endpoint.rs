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
pub struct IrohEndpoint {
    base: Base<Object>,
    pub(crate) endpoint: Arc<Mutex<Option<Endpoint>>>,
}

#[godot_api]
impl IObject for IrohEndpoint {
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
    fn bind_async_result(result: bool);

    // #endregion

    // #region connect

    #[func]
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
    fn connect_async_result(result: Option<Gd<IrohConnection>>);

    // #endregion

    // #region accept

    #[func]
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
    fn accept_async_result(result: Option<Gd<IrohConnection>>);

    // #endregion

    #[func]
    fn addr(&self) -> GString {
        let Some(ref endpoint) = *self.endpoint.lock().unwrap() else {
            godot_error!("Endpoint is not initialized yet");
            return GString::from("N/A");
        };

        let key = endpoint.addr().id.to_string();
        return GString::from(key.as_str());
    }
}
