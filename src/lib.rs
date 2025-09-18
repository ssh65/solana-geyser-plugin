pub mod plugin;
pub mod server;
pub mod service;

pub mod geyser {
    tonic::include_proto!("geyser");
}

pub use crate::plugin::GeyserGrpcPlugin;
pub use crate::server::start_grpc_server;
pub use crate::service::MyGeyserService;

// Used by validator to create the plugin
#[no_mangle]
pub unsafe extern "C" fn _create_plugin()
-> *mut dyn solana_geyser_plugin_interface::geyser_plugin_interface::GeyserPlugin {
    Box::into_raw(Box::new(GeyserGrpcPlugin {
        update_sender: None,
    }))
}
