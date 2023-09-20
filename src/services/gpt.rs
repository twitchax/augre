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

Please prioritize the response by impact to the code, and please split the suggestions into three categories:
1. Suggestions that pertain to likely runtime bugs or errors.
2. Suggestions that pertain to likely logic bugs or errors.
3. Suggestions that pertain to likely style bugs or errors.

If possible, please also provide a suggested fix to the identified issue.  If you are unable to provide a suggested fix, please provide a reason why.
"#;