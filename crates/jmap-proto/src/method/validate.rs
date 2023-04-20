use serde::Serialize;

use crate::{
    error::set::SetError,
    parser::{json::Parser, JsonObjectParser, Token},
    request::RequestProperty,
    types::{blob::BlobId, id::Id},
};

#[derive(Debug, Clone)]
pub struct ValidateSieveScriptRequest {
    pub account_id: Id,
    pub blob_id: BlobId,
}

#[derive(Debug, Serialize)]
pub struct ValidateSieveScriptResponse {
    #[serde(rename = "accountId")]
    pub account_id: Id,
    pub error: Option<SetError>,
}

impl JsonObjectParser for ValidateSieveScriptRequest {
    fn parse(parser: &mut Parser<'_>) -> crate::parser::Result<Self>
    where
        Self: Sized,
    {
        let mut request = ValidateSieveScriptRequest {
            account_id: Id::default(),
            blob_id: BlobId::default(),
        };

        parser
            .next_token::<String>()?
            .assert_jmap(Token::DictStart)?;

        while let Some(key) = parser.next_dict_key::<RequestProperty>()? {
            match &key.hash[0] {
                0x6449_746e_756f_6363_61 if !key.is_ref => {
                    request.account_id = parser.next_token::<Id>()?.unwrap_string("accountId")?;
                }
                0x6449_626f_6c62 if !key.is_ref => {
                    request.blob_id = parser.next_token::<BlobId>()?.unwrap_string("blobId")?;
                }
                _ => {
                    parser.skip_token(parser.depth_array, parser.depth_dict)?;
                }
            }
        }

        Ok(request)
    }
}
