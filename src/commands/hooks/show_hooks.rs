use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

impl<'a> TmuxInterface<'a> {
    const SHOW_HOOK: &'static str = "show-hooks";
    /// # Manual
    ///
    /// tmux ^2.2:
    /// ```text
    /// tmux show-hooks [-g] [-t target-session]
    /// ```
    pub fn show_hooks(
        &mut self,
        global: Option<bool>,
        target_session: Option<&'a str>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if global.unwrap_or(false) {
            args.push(g_KEY);
        }
        if let Some(target_session) = target_session {
            args.extend_from_slice(&[t_KEY, &target_session])
        }
        let output = self.command(TmuxInterface::SHOW_HOOK, &args)?;
        Ok(output)
    }
}