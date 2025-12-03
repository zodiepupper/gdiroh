use godot::prelude::*;

pub(crate) fn create_signal_emitter<C, T>(
    instance_id: InstanceId,
    signal_name: &'static str,
) -> impl Fn(T)
where
    C: GodotClass + Inherits<Object>,
    T: ToGodot + 'static,
{
    move |result: T| {
        if let Ok(gd_self) = Gd::<C>::try_from_instance_id(instance_id) {
            let mut gd_object = gd_self.upcast::<Object>();
            gd_object.call_deferred(
                "emit_signal",
                &[signal_name.to_variant(), result.to_variant()],
            );
        } else {
            godot_warn!("IrohEndpoint freed before '{signal_name}' signal could be emitted");
        }
    }
}
