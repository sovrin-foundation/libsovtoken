use sovtoken::logic::parsers::common::ResponseOperations;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ParseMintResponse {
    pub op : ResponseOperations,
}