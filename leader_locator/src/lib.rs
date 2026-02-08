use serde::Serialize;
use zela_std::{CustomProcedure, RpcError, rpc_client::RpcClient, zela_custom_procedure};

pub mod location;

pub struct LeaderLocator;

#[derive(Serialize)]
pub struct Output {
    slot: u64,
    leader: String,
    leader_geo: String,
    closest_region: String,
}

impl CustomProcedure for LeaderLocator {
    type Params = ();
    type ErrorData = ();
    type SuccessData = Output;

    async fn run(_params: Self::Params) -> Result<Self::SuccessData, RpcError<Self::ErrorData>> {
        let client = RpcClient::new();
        let slot = client.get_slot().await?;

        let slot_leader = client
            .get_slot_leaders(slot, 1)
            .await?
            .first()
            .ok_or(RpcError {
                code: 500,
                message: format!("No leaders found for slot: {}", slot),
                data: None,
            })?
            .to_string();

        let cluster_nodes = client.get_cluster_nodes().await?;

        let leader_contact_info = cluster_nodes
            .iter()
            .find(|contact_info| contact_info.pubkey == slot_leader)
            .ok_or(RpcError {
                code: 500,
                message: format!("No contact_info found for leader: {}", slot_leader),
                data: None,
            })?;

        let (closest_region, leader_geo) = location::get_geo_info(leader_contact_info, slot)?;

        Ok(Output {
            slot,
            leader: slot_leader,
            closest_region: closest_region.to_string(),
            leader_geo: leader_geo.to_string(),
        })
    }
}

zela_custom_procedure!(LeaderLocator);
