use std::{str::FromStr, time::Duration};

use chatgpt::prelude::{ChatGPT, ModelConfiguration, ChatGPTEngine};
use tokio::process::Command;
use anyhow::{Result, Context};
use url::Url;

use crate::base::types::{HasName, IsEnsurable, is_binary_present, MapStatus, Mode, Res, Void};

static NAME: &str = "gpt";

pub struct Gpt {
    url: String,
    key: Option<String>,
    mode: Mode,
}

impl HasName for Gpt {
    fn name(&self) -> &'static str {
        NAME
    }
}

impl IsEnsurable for Gpt {
    async fn is_present(&self) -> Res<bool> {
        let _ = self.resolve_key()?;
        
        Ok(true)
    }

    async fn make_present(&self) -> Void {
        Err(anyhow::Error::msg("Cannot perform `make_present`: this should not happen."))
    }
}

impl Gpt {
    pub async fn review(&self, diff: &str) -> Res<String> {
        let key = self.resolve_key()?;

        let url = format!("{}/v1/chat/completions", self.url);
        let config = ModelConfiguration {
            engine: ChatGPTEngine::Gpt4,
            api_url: Url::from_str(&url)?,
            timeout: Duration::from_secs(120),
            ..Default::default()
        };

        let client = ChatGPT::new_with_config(key, config)?;

        let message = REVIEW_PROMPT.replace("{{diff}}", diff);

        let response = client.send_message(message).await?;

        Ok(response.message_choices[0].message.content.clone())
    }

    pub async fn ask(&self, prompt: &str) -> Res<String> {
        let key = self.resolve_key()?;

        let url = format!("{}/v1/chat/completions", self.url);
        let config = ModelConfiguration {
            engine: ChatGPTEngine::Gpt4,
            api_url: Url::from_str(&url)?,
            timeout: Duration::from_secs(120),
            ..Default::default()
        };

        let client = ChatGPT::new_with_config(key, config)?;

        let response = client.send_message(prompt.to_string()).await?;

        Ok(response.message_choices[0].message.content.clone())
    }
}

impl Gpt {
    pub fn new(url: &str, key: &Option<String>, mode: Mode) -> Self {
        Self {
            url: url.to_string(),
            key: key.clone(),
            mode,
        }
    }

    fn resolve_key(&self) -> Res<&str> {
        let key = if self.mode == Mode::OpenAi {
            self.key.as_ref().ok_or(anyhow::Error::msg("OpenAI key not provided.  Please set the `openai_key` config value, or use a local mode."))?
        } else {
            ""
        };

        Ok(key)
    }
}

// Statics.

static REVIEW_PROMPT: &str = r#"
Please perform a code review of the following diff (produced by `git diff` on my code), and provide suggestions for improvement:

```
{{diff}}
```

Please prioritize the response by impact to the code, and please split the suggestions into three categories:
1. Suggestions that pertain to likely runtime bugs or errors.
2. Suggestions that pertain to likely logic bugs or errors.
3. Suggestions that pertain to likely style bugs or errors.

If possible, please also provide a suggested fix to the identified issue.  If you are unable to provide a suggested fix, please provide a reason why.

The format should look like:

```
1. Likely runtime bugs:
- Some suggestion...
- Another...

2. Likely logic bugs:
- Suggestion 1
- Suggestion 2
- Suggestion 3

3. Likely style bugs:
- Suggestion 1
- Suggestion 2
- Suggestion 3
```

For each relevant code snippet, please provide context about where the suggestion is relevant (e.g., `path/file.rs:30`); in addition, if a code snippet would be helpful, please provide a code snippet showing the fix.
"#;