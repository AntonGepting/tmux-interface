use crate::commands::constants::TMUX;
use crate::TmuxOutput;
use std::borrow::Cow;
use std::process::{Command, Stdio};

/// Standard tmux command line arguments syntax:
/// ```text
/// tmux [-2CluvV] [-c shell-command] [-f file] [-L socket-name] [-S socket-path] [command [flags]]
/// ```
///
/// Tmux command line parts:
/// - executable (part I) (example: `tmux`)
/// - executable args (part II) (example: `[-2CluvV] [-c shell-command] [-f file] [-L socket-name] [-S socket-path]`)
/// - command (part III) (example: `[command]`)
/// - command args (part IV) (example: `[flags]`)
#[derive(Debug, Clone)]
pub struct TmuxCommand<'a> {
    // XXX: rename tmux?
    /// Tmux executable name, (part I) if is `None`, will be used `tmux`
    pub bin: Cow<'a, str>,
    /// Tmux executable arguments (part II)
    pub bin_args: Option<Vec<String>>,
    /// Tmux command (part III)
    pub cmd: Option<Cow<'a, str>>,
    /// Tmux command arguments (part IV)
    pub cmd_args: Option<Vec<String>>,
}

impl<'a> Default for TmuxCommand<'a> {
    fn default() -> Self {
        TmuxCommand {
            bin: Cow::Borrowed(TMUX),
            bin_args: None,
            cmd: None,
            cmd_args: None,
        }
    }
}

impl<'a> TmuxCommand<'a> {
    pub fn bin<S: Into<Cow<'a, str>>>(&mut self, bin: S) -> &mut Self {
        //self.tmux.bin = bin;
        self.bin = bin.into();
        self
    }

    pub fn cmd<S: Into<Cow<'a, str>>>(&mut self, cmd: S) -> &mut Self {
        self.cmd = Some(cmd.into());
        self
    }

    // if we are working with same type problems multiple traits methods mixing allowed (NewSession, DetachClient, chaining methods)

    //// NOTE: inherit stdin to prevent tmux fail with error `terminal failed: not a terminal`
    //cmd.stdin(Stdio::inherit());
    /// run command
    pub fn output(&self) -> TmuxOutput {
        let mut command = self.push_tmux_command();
        println!("{:?}", &self);
        command.stdin(Stdio::inherit());
        let output = command.output().unwrap();
        println!("{:?}", output);
        TmuxOutput(output)
    }

    pub fn push_tmux_command(&self) -> Command {
        let mut command = Command::new(&self.bin.as_ref());

        // XXX: ugly?
        if let Some(s) = &self.bin_args {
            command.args(s);
        }
        if let Some(s) = &self.bin_args {
            command.args(s);
        }

        if let Some(s) = &self.cmd {
            command.arg(&**s);
        }

        // XXX: ugly?
        if let Some(s) = &self.cmd_args {
            command.args(s);
        }

        command
    }

    //pub fn spawn(&self) -> Result<Child> {
    //let mut command = self.push_tmux_command();
    //command.spawn()
    //}

    // XXX: hard bound to cmd_args
    // if vec doesn't exist, creates it and appends with given arguments
    /// insert a single flag (`-x`)
    pub fn push_flag<S: Into<String>>(&mut self, flag: S) -> &mut Self {
        self.push_param(flag.into())
    }

    // if vec doesn't exist, creates it and appends with given arguments
    /// insert an option, flag and value (`-x  <VALUE>`)
    pub fn push_option<S, U>(&mut self, key: S, option: U) -> &mut Self
    where
        S: Into<String>,
        U: Into<String>,
    {
        self.cmd_args
            .get_or_insert(Vec::new())
            .extend_from_slice(&[key.into(), option.into()]);
        self
    }

    // if vec doesn't exist, creates it and appends with given arguments
    /// insert a single parameter (`[VALUE]`)
    pub fn push_param<S: Into<String>>(&mut self, param: S) -> &mut Self {
        self.cmd_args.get_or_insert(Vec::new()).push(param.into());
        self
    }

    pub fn new() -> Self {
        Default::default()
    }

    //pub fn output(&self) -> Ou {

    //}
}
