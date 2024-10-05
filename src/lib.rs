pub mod file;
pub mod pkg;
pub mod log;

use fls::prelude::FLSMsg;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct TestMessage
{
    pub str : String
}

impl FLSMsg for TestMessage {
    
}