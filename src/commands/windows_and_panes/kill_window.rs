use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

impl<'a> TmuxInterface<'a> {
    #[cfg(not(feature = "use_cmd_alias"))]
    const KILL_WINDOW: &'static str = "kill-window";
    #[cfg(feature = "use_cmd_alias")]
    const KILL_WINDOW: &'static str = "killw";

    /// Kill the current window or the window at target-window, removing it from any sessions
    /// to which it is linked
    ///
    /// # Manual
    ///
    /// tmux ^1.7:
    /// ```text
    /// tmux kill-window [-a] [-t target-window]
    /// (alias: killw)
    /// ```
    ///
    /// tmux ^0.8:
    /// ```text
    /// tmux kill-window [-a] [-t target-window]
    /// (alias: killw)
    /// ```
    pub fn kill_window(
        &mut self,
        all: Option<bool>,
        target_window: Option<&str>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        #[cfg(feature = "tmux_1_7")]
        if all.unwrap_or(false) {
            args.push(a_KEY);
        }
        #[cfg(feature = "tmux_0_8")]
        if let Some(target_window) = target_window {
            args.extend_from_slice(&[t_KEY, &target_window]);
        }
        let output = self.command(TmuxInterface::KILL_WINDOW, &args)?;
        Ok(output)
    }
}