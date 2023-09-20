use anyhow::Result;
use tokio::process::Command;
use std::process::{Stdio, ExitStatus};
use yansi::Paint;
use dialoguer::Confirm;

// Statics.

static TAB: &str = "  ";

// Error helpers.

pub type Res<T> = Result<T, anyhow::Error>;
pub type Void = Res<()>;

// Traits for various internal functionality.

pub trait Nameable {
    fn name(&self) -> &'static str;
}

pub trait Ensurable {
    async fn is_present(&self) -> Result<bool>;
    async fn make_present(&self) -> Result<()>;
}

pub trait Removable {
    async fn make_not_present(&self) -> Result<()>;
}

// "Public" Entity definitions (performs passthrough to helper functions).

pub trait EnsurableEntity {
    async fn ensure(&self, confirm: bool) -> Result<()>;
}

impl<T> EnsurableEntity for T
    where T: Nameable + Ensurable + Send + Sync
{
    async fn ensure(&self, confirm: bool) -> Result<()> {
        let name = self.name();
        print!("Checking if `{}` is present ... ", Paint::blue(name));

        if self.is_present().await? {
            println!("💯!");
            return Ok(())
        }

        println!("{}!", Paint::red("✘"));
        
        if confirm && !Confirm::new().with_prompt(format!("{}`{}` is not present: do you want me to make it so?", TAB, Paint::blue(name))).interact()? {
            println!("{}Skipping ...", TAB);
            return Ok(())
        }
        
        println!("{}Ensuring presence of `{}` ({}) ...", TAB, Paint::blue(name), Paint::yellow("you may need to interact with the execution"));
        
        self.make_present().await?;
        
        println!("{}Successfully ensured `{}`.", TAB, Paint::blue(name));
        
        Ok(())
    }
}

pub trait RemovableEntity {
    async fn remove(&self, confirm: bool) -> Result<()>;
}

impl<T> RemovableEntity for T
    where T: Nameable + Ensurable + Removable + Send + Sync
{
    async fn remove(&self, confirm: bool) -> Result<()> {
        let name = self.name();

        if !self.is_present().await? {
            println!("{} `{}` is not running!", TAB, Paint::blue(name));
            return Ok(())
        }

        if confirm && !Confirm::new().with_prompt(format!("{}`{}` is present: do you want me to remove it?", TAB, Paint::blue(name))).interact()? {
            println!("{}Skipping ...", TAB);
            return Ok(())
        }

        println!("{}Removing presence of `{}` ({}) ...", TAB, Paint::blue(name), Paint::yellow("you may need to interact with the execution [and sudo]"));
        
        self.make_not_present().await?;
        
        println!("{}Successfully removed `{}`.", TAB, Paint::blue(name));
        
        Ok(())
    }
}

// Exit status helper trait.

pub trait MapStatus {
    fn map_status(self) -> Result<()>;
}

impl MapStatus for Result<ExitStatus, std::io::Error> {
    fn map_status(self) -> Result<()> {
        self
            .map(|s| s.success())
            .map_err(|e| e.into())
            .and_then(|s| if s { Ok(()) } else { Err(anyhow::Error::msg("The exit code of the operation was not successful.")) })
    }
}

// Other helper methods.

pub(crate) async fn is_binary_present<T>(s: &T) -> Result<bool>
    where T: Nameable
{
    Ok(Command::new("which")
        .arg(s.name())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .stdin(Stdio::null())
        .status().await?.success())
}