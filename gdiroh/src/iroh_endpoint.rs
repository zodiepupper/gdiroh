use godot::prelude::*;
use iroh::Endpoint;

use crate::runtime::IrohRuntime;

#[derive(GodotClass)]
#[class(base=Object)]
pub struct IrohEndpoint {
    base: Base<Object>,
    pub(crate) endpoint: Option<Endpoint>,
}

#[godot_api]
impl IObject for IrohEndpoint {
    fn init(base: Base<Object>) -> Self {
        Self {
            base,
            endpoint: None,
        }
    }
}

#[godot_api]
impl IrohEndpoint {
    #[func]
    fn bind(&mut self) {
        if self.endpoint.is_some() {
            godot_warn!("Called `bind` on a non empty IrohEndpoint, overwriting...");
        }

        let endpoint = IrohRuntime::block_on(async move {
            Endpoint::bind()
                .await
                .expect("Failed to bind iroh Endpoint")
        });

        godot_print!("Endpoint at `{}`", endpoint.addr().id);

        self.endpoint = Some(endpoint);
    }
}
