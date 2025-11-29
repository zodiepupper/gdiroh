/*
File modified from: https://github.com/tipragot/godot-iroh/blob/main/src/lib.rs

```
MIT License

Copyright (c) 2025 Tipragot

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```
*/

use std::time::Duration;

use godot::{
    classes::Engine,
    prelude::*,
};
use tokio::{
    runtime::{
        self,
        Runtime,
    },
    task::JoinHandle,
};

#[derive(GodotClass)]
#[class(base=Object)]
pub struct IrohRuntime {
    base: Base<Object>,
    runtime: Option<Runtime>,
}

#[godot_api]
impl IObject for IrohRuntime {
    fn init(base: Base<Object>) -> Self {
        let runtime = Some(runtime::Builder::new_multi_thread().enable_all().build().unwrap());

        Self { base, runtime }
    }
}

#[godot_api]
impl IrohRuntime {
    pub fn spawn<F>(future: F) -> JoinHandle<F::Output>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
        Engine::singleton()
            .get_singleton("IrohRuntime")
            .expect("singleton not found")
            .cast::<Self>()
            .bind()
            .runtime
            .as_ref()
            .expect("invalid singleton")
            .spawn(future)
    }

    pub fn block_on<F: Future>(future: F) -> F::Output {
        Engine::singleton()
            .get_singleton("IrohRuntime")
            .expect("singleton not found")
            .cast::<Self>()
            .bind()
            .runtime
            .as_ref()
            .expect("invalid singleton")
            .block_on(future)
    }

    pub fn init_singleton(level: &InitLevel) {
        if level == &InitLevel::Scene {
            Engine::singleton().register_singleton("IrohRuntime", &IrohRuntime::new_alloc());
        }
    }

    pub fn deinit_singleton(level: &InitLevel) {
        if level == &InitLevel::Scene {
            let mut engine = Engine::singleton();
            let singleton = engine.get_singleton("IrohRuntime").expect("Iroh singleton not found");
            engine.unregister_singleton("IrohRuntime");
            singleton.free();
        }
    }
}

impl Drop for IrohRuntime {
    fn drop(&mut self) {
        if let Some(runtime) = std::mem::take(&mut self.runtime) {
            runtime.shutdown_timeout(Duration::from_secs(5));
        }
    }
}
