#![allow(dead_code)]

use godot::prelude::*;

use crate::runtime::IrohRuntime;

mod iroh_endpoint;
mod iroh_gossip;
mod iroh_router;
mod runtime;

struct GDIrohExtension;

#[gdextension]
unsafe impl ExtensionLibrary for GDIrohExtension {
    fn on_level_init(level: InitLevel) {
        IrohRuntime::init_singleton(&level);
    }

    fn on_level_deinit(level: InitLevel) {
        IrohRuntime::deinit_singleton(&level);
    }
}
