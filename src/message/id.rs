use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct MessageID(pub String);
