use tokio::process::Command;
use anyhow::{Result, Context};

use crate::base::types::{Nameable, Ensurable, is_binary_present, MapStatus};

static NAME: &str = "gpt";

#[derive(Default)]
pub struct Gpt {}

impl Nameable for Gpt {
    fn name(&self) -> &'static str {
        NAME
    }
}

impl Ensurable for Gpt {
    async fn is_present(&self) -> Result<bool> {
        Ok(true)
    }

    async fn make_present(&self) -> Result<()> {
        Ok(())
    }
}

impl Gpt {
    pub async fn review(key: &str, diff: &str) -> Result<String> {
        
    }
}

// Statics.

static REVIEW_PROMPT: &str = r#"
Please perform a code review of the following diff:

{{diff}}


"#;