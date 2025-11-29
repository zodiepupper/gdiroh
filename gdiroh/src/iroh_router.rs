use godot::prelude::*;
use iroh::protocol::Router;

use crate::{
    iroh_endpoint::IrohEndpoint,
    iroh_gossip::IrohGossip,
    runtime::IrohRuntime,
};

#[derive(GodotClass)]
#[class(base=Object)]
struct IrohRouter {
    base: Base<Object>,
    pub(crate) router: Option<Router>,
}

#[godot_api]
impl IObject for IrohRouter {
    fn init(base: Base<Object>) -> Self {
        Self { base, router: None }
    }
}

#[godot_api]
impl IrohRouter {
    #[func]
    // TODO: Add generic ALPN and handler arguments so this isn't directly bound to iroh-gossip
    fn bind(&mut self, endpoint: Gd<IrohEndpoint>, gossip: Gd<IrohGossip>) {
        if self.router.is_some() {
            godot_warn!("Called `bind` on a non empty IrohRouter, overwriting...");
        }

        let Some(real_endpoint) = endpoint.bind().endpoint.clone() else {
            godot_error!(
                "Attempted to bind a IrohRouter object with a uninitialized IrohEndpoint as \
                 input, you must first bind the endpoint!"
            );
            return;
        };

        let Some(real_gossip) = gossip.bind().gossip.clone() else {
            godot_error!(
                "Attempted to bind a IrohRouter object with a uninitialized IrohGossip as input, \
                 you must first spawn the gossip!"
            );
            return;
        };

        let router = IrohRuntime::block_on(async move {
            Router::builder(real_endpoint)
                .accept(iroh_gossip::ALPN, real_gossip)
                .spawn()
        });

        self.router = Some(router);
    }
}
