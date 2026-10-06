use anyhow::{bail, Result};

#[derive(Clone, Debug, Default)]
pub struct IssuanceInfo {
    pub isk: Vec<u8>,
    pub first_issuance: bool,
}

pub async fn issue_asset(
    _asset_name: String,
    _amount: u64,
    _first_issuance: bool,
    _finalize: bool,
    _desc_hash: Option<Vec<u8>>,
    _id_account: u32,
    _c: &crate::api::coin::Coin,
) -> Result<()> {
    bail!("ZSA issuance is not part of public NU7")
}
