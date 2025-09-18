use crate::geyser;
use crate::server::start_grpc_server;
use log::*;
use solana_geyser_plugin_interface::geyser_plugin_interface::*;
use tokio::sync::mpsc;

#[derive(Debug)]
pub struct GeyserGrpcPlugin {
    pub updates_sender: Option<mpsc::UnboundedSender<geyser::UpdateMessage>>,
}

#[allow(unused_variables)]
impl GeyserPlugin for GeyserGrpcPlugin {
    fn setup_logger(&self, logger: &'static dyn log::Log, level: log::LevelFilter) -> Result<()> {
        // customize as needed
        Ok(())
    }

    fn name(&self) -> &'static str {
        "GeyserGrpcPlugin"
    }

    fn on_load(&mut self, _config_file: &str, _is_reload: bool) -> Result<()> {
        info!("Plugin loaded");
        let (tx, rx) = mpsc::unbounded_channel();
        self.updates_sender = Some(tx);

        std::thread::spawn(move || {
            let rt = tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap();
            rt.block_on(async move {
                start_grpc_server(rx).await;
            });
        });

        Ok(())
    }

    fn on_unload(&mut self) {
        info!("Plugin unloaded");
    }

    fn update_account(
        &self,
        account: ReplicaAccountInfoVersions,
        slot: u64,
        _is_startup: bool,
    ) -> Result<()> {
        if let Some(sender) = &self.updates_sender {
            let (pubkey, lamports, owner, data) = match &account {
                ReplicaAccountInfoVersions::V0_0_1(acc) => (
                    bs58::encode(acc.pubkey).into_string(), // convert [u8; 32] to base58 string
                    acc.lamports,
                    bs58::encode(acc.owner).into_string(), // convert owner to base58 string
                    acc.data.to_vec(),
                ),

                ReplicaAccountInfoVersions::V0_0_2(acc) => (
                    bs58::encode(acc.pubkey).into_string(),
                    acc.lamports,
                    bs58::encode(acc.owner).into_string(),
                    acc.data.to_vec(),
                ),

                ReplicaAccountInfoVersions::V0_0_3(acc) => (
                    bs58::encode(acc.pubkey).into_string(),
                    acc.lamports,
                    bs58::encode(acc.owner).into_string(),
                    acc.data.to_vec(),
                ),
            };

            let account_update = geyser::AccountUpdate {
                pubkey,
                lamports,
                owner,
                data,
                slot,
            };

            let update_msg = geyser::UpdateMessage {
                update: Some(geyser::update_message::Update::AccountUpdate(
                    account_update,
                )),
            };

            let _ = sender.send(update_msg);
        }
        Ok(())
    }

    fn notify_transaction(
        &self,
        transaction: ReplicaTransactionInfoVersions,
        slot: u64,
    ) -> Result<()> {
        // Logic as needed
        Ok(())
    }

    fn update_slot_status(&self, slot: u64, parent: Option<u64>, status: SlotStatus) -> Result<()> {
        // Logic as needed
        Ok(())
    }

    fn notify_end_of_startup(&self) -> Result<()> {
        Ok(())
    }

    fn notify_entry(&self, entry: ReplicaEntryInfoVersions) -> Result<()> {
        Ok(())
    }

    fn notify_block_metadata(&self, blockinfo: ReplicaBlockInfoVersions) -> Result<()> {
        Ok(())
    }

    fn account_data_notifications_enabled(&self) -> bool {
        true
    }
    fn transaction_notifications_enabled(&self) -> bool {
        true
    }
    fn entry_notifications_enabled(&self) -> bool {
        false
    }
}
