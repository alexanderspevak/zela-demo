use chrono::Utc;
use serde::Serialize;
use zela_std::{
    CustomProcedure, JsonValue, RpcError, rpc_client::RpcClient, zela_custom_procedure,
};

const SLOT_DURATION_MILLIS: i64 = 400;
pub struct LeaderLocator;

#[derive(Serialize)]
pub struct Output {
    pub output: String,
}

impl CustomProcedure for LeaderLocator {
    type Params = ();
    type ErrorData = JsonValue;
    type SuccessData = Output;

    async fn run(_params: Self::Params) -> Result<Self::SuccessData, RpcError<Self::ErrorData>> {
        let client = RpcClient::new();

        let slot = client.get_slot().await?;
        let time_start = Utc::now();
        let slot_leaders = client.get_slot_leaders(slot, 2).await?;
        let time_end = Utc::now();

        if slot_leaders.is_empty() {
            return Err(RpcError {
                code: 500,
                message: format!("No leaders found for slot: {}", slot),
                data: None,
            });
        }

        let cluster_nodes = client.get_cluster_nodes().await?;

        let time_wait_millis = (time_start - time_end).num_milliseconds();

        let probable_leader = if time_wait_millis < SLOT_DURATION_MILLIS / 2 {
            slot_leaders.first().expect(
                "First slot leader should be available as slot leader vec should have length",
            )
        } else {
            match slot_leaders.get(1) {
                Some(leader) => leader,
                None => {
                    log::warn!("Most probable slot leader was not present");

                    slot_leaders
                        .first()
                        .expect("Slot leaders vector should have items")
                }
            }
        }
        .to_string();

        let leader_contact_info = cluster_nodes
            .iter()
            .find(|contact_info| contact_info.pubkey == probable_leader)
            .ok_or(RpcError {
                code: 500,
                message: format!("No contact_info found for leader: {}", probable_leader),
                data: None,
            })?;

        log::info!("Procedure time {}", time_wait_millis);
        log::info!("Leader ip {}", leader_contact_info.gossip.unwrap().ip());
        Ok(Output {
            output: "abc".to_string(),
        })
    }
}

zela_custom_procedure!(LeaderLocator);
