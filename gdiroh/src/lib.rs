#![allow(dead_code)]

use godot::prelude::*;

use crate::runtime::IrohRuntime;

mod connection;
mod endpoint;
mod helpers;
mod runtime;
mod streams;

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
