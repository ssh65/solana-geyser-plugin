use solana_geyser_plugin_interface::geyser_plugin_interface::GeyserPlugin;

pub mod plugin;
pub mod server;
pub mod service;
pub mod geyser {
    tonic::include_proto!("geyser");
}

pub use crate::plugin::GeyserGrpcPlugin;

// Used by validator to create the plugin
#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn _create_plugin() -> *mut dyn GeyserPlugin {
    let plugin = Box::new(GeyserGrpcPlugin {
        updates_sender: None,
    });
    Box::into_raw(plugin)
}
