use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct VerificationMessage {
    pub email: String,
    pub verification_link: String,
}
