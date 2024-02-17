#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Default)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct GetCompressedAccountProofRequest {
    pub hash: Option<String>,
    pub account_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Default)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct GetCompressedAccountProofResponse {
    pub hash: String,
    pub proof: Vec<String>,
    pub seq: i64,
    pub slot_updated: i64,
}

pub fn get_compressed_account_proof(
    conn: &DatabaseConnection,
    request: GetCompressedAccountProofRequest,
) -> Result<GetCompressedAccountProofResponse, InternalApiError> {
    todo!();
}
