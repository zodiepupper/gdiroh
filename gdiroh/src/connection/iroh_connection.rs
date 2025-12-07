use std::sync::{
    Arc,
    Mutex,
};

use godot::prelude::*;
use iroh::endpoint::Connection;

use crate::{
    helpers::create_signal_emitter,
    runtime::IrohRuntime,
    streams::IrohStream,
};

#[derive(GodotClass)]
#[class(base=Object)]
pub struct IrohConnection {
    base: Base<Object>,
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
    fn open_uni_blocking(&self) -> Option<Gd<IrohStream>> {
        let connection_ref = self.connection.clone();

        let result = IrohRuntime::block_on(Self::open_uni_logic(connection_ref));

        match result {
            Ok(iroh_stream) => {
                return Some(iroh_stream);
            }
            Err(err) => {
                godot_error!("Opening connection failed: {err}");
                return None;
            }
        }
    }

    #[func]
    fn open_uni_async(&self) {
        let connection_ref = self.connection.clone();
        let instance_id = self.base().instance_id();

        IrohRuntime::spawn(async move {
            let push_result = create_signal_emitter::<Self, Option<Gd<IrohStream>>>(
                instance_id,
                "open_uni_async_results",
            );

            let result = Self::open_uni_logic(connection_ref).await;

            match result {
                Ok(iroh_stream) => {
                    push_result(Some(iroh_stream));
                }
                Err(err) => {
                    godot_error!("Opening connection failed: {err}");
                }
            }
        });
    }

    #[signal]
    fn open_uni_async_results(result: Option<Gd<IrohStream>>);

    // #endregion

    // #region open_bi

    #[func]
    fn open_bi_blocking(&self) -> Option<Gd<IrohStream>> {
        let connection_ref = self.connection.clone();

        let result = IrohRuntime::block_on(Self::open_bi_logic(connection_ref));

        match result {
            Ok(iroh_stream) => {
                return Some(iroh_stream);
            }
            Err(err) => {
                godot_error!("Opening connection failed: {err}");
                return None;
            }
        }
    }

    #[func]
    fn open_bi_async(&self) {
        let connection_ref = self.connection.clone();
        let instance_id = self.base().instance_id();

        IrohRuntime::spawn(async move {
            let push_result = create_signal_emitter::<Self, Option<Gd<IrohStream>>>(
                instance_id,
                "open_bi_async_results",
            );

            let result = Self::open_bi_logic(connection_ref).await;

            match result {
                Ok(iroh_stream) => {
                    push_result(Some(iroh_stream));
                }
                Err(err) => {
                    godot_error!("Opening connection failed: {err}");
                }
            }
        });
    }

    #[signal]
    fn open_bi_async_results(result: Option<Gd<IrohStream>>);

    // #endregion

    // #region accept_uni

    #[func]
    fn accept_uni_blocking(&self) -> Option<Gd<IrohStream>> {
        let connection_ref = self.connection.clone();

        let result = IrohRuntime::block_on(Self::accept_uni_logic(connection_ref));

        match result {
            Ok(iroh_stream) => {
                return Some(iroh_stream);
            }
            Err(err) => {
                godot_error!("Accepting connection failed: {err}");
                return None;
            }
        }
    }

    #[func]
    fn accept_uni_async(&self) {
        let connection_ref = self.connection.clone();
        let instance_id = self.base().instance_id();

        IrohRuntime::spawn(async move {
            let push_result = create_signal_emitter::<Self, Option<Gd<IrohStream>>>(
                instance_id,
                "accept_uni_async_results",
            );

            let result = Self::accept_uni_logic(connection_ref).await;

            match result {
                Ok(iroh_stream) => {
                    push_result(Some(iroh_stream));
                }
                Err(err) => {
                    godot_error!("Accepting connection failed: {err}");
                }
            }
        });
    }

    #[signal]
    fn accept_uni_async_results(result: Option<Gd<IrohStream>>);

    // #endregion

    // #region accept_bix

    #[func]
    fn accept_bi_blocking(&self) -> Option<Gd<IrohStream>> {
        let connection_ref = self.connection.clone();

        let result = IrohRuntime::block_on(Self::accept_bi_logic(connection_ref));

        match result {
            Ok(iroh_stream) => {
                return Some(iroh_stream);
            }
            Err(err) => {
                godot_error!("Accepting connection failed: {err}");
                return None;
            }
        }
    }

    #[func]
    fn accept_bi_async(&self) {
        let connection_ref = self.connection.clone();
        let instance_id = self.base().instance_id();

        IrohRuntime::spawn(async move {
            let push_result = create_signal_emitter::<Self, Option<Gd<IrohStream>>>(
                instance_id,
                "accept_bi_async_results",
            );

            let result = Self::accept_bi_logic(connection_ref).await;

            match result {
                Ok(iroh_stream) => {
                    push_result(Some(iroh_stream));
                }
                Err(err) => {
                    godot_error!("Accepting connection failed: {err}");
                }
            }
        });
    }

    #[signal]
    fn accept_bi_async_results(result: ());

    // #endregion
}
