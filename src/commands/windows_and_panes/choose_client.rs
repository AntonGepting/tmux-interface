use crate::error::Error;
use crate::tmux_interface::*;
use std::process::Output;

/// Put a pane into client mode, allowing a client to be selected interactively from a list
///
/// # Manual
///
/// tmux ^3.1:
/// ```text
/// tmux choose-client [-NrZ] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
/// ```
///
/// tmux ^2.7:
/// ```text
/// tmux choose-client [-NZ] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
/// ```
///
/// tmux ^2.6:
/// ```text
/// tmux choose-client [-N] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
/// ```
///
/// tmux ^1.7:
/// ```text
/// tmux choose-client [-F format] [-t target-window] [template]
/// ```
///
/// tmux ^1.0:
/// ```text
/// tmux choose-client  [-t target-window] [template]
/// ```
#[derive(Default, Debug)]
pub struct ChooseClient<'a> {
    /// [-N] - start without the preview
    #[cfg(feature = "tmux_2_6")]
    pub without_preview: Option<bool>,
    /// [-r] - reverse the sort order
    #[cfg(feature = "tmux_3_1")]
    pub reverse_sort_order: Option<bool>,
    /// [-Z] - zoom the pane
    #[cfg(feature = "tmux_3_1")]
    pub zoom: Option<bool>,
    /// [-F format] - format
    #[cfg(feature = "tmux_1_7")]
    pub format: Option<&'a str>,
    /// [-f filter] - specify an initial filter
    #[cfg(feature = "tmux_2_6")]
    pub filter: Option<&'a str>,
    /// [-O sort-order] - specify the initial sort field
    #[cfg(feature = "tmux_2_6")]
    pub sort_order: Option<&'a str>,
    /// [-t target-pane] - target-pane
    #[cfg(feature = "tmux_2_6")]
    pub target_pane: Option<&'a str>,
    /// [-t target-window] - target-window
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_6")))]
    pub target_window: Option<&'a str>,
    /// [template] - template
    #[cfg(feature = "tmux_1_0")]
    pub template: Option<&'a str>,
}

impl<'a> ChooseClient<'a> {
    pub fn new() -> Self {
        Default::default()
    }
}

#[derive(Default, Debug)]
pub struct ChooseClientBuilder<'a> {
    #[cfg(feature = "tmux_2_6")]
    pub without_preview: Option<bool>,
    #[cfg(feature = "tmux_3_1")]
    pub reverse_sort_order: Option<bool>,
    #[cfg(feature = "tmux_3_1")]
    pub zoom: Option<bool>,
    #[cfg(feature = "tmux_1_7")]
    pub format: Option<&'a str>,
    #[cfg(feature = "tmux_2_6")]
    pub filter: Option<&'a str>,
    #[cfg(feature = "tmux_2_6")]
    pub sort_order: Option<&'a str>,
    #[cfg(feature = "tmux_2_6")]
    pub target_pane: Option<&'a str>,
    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_6")))]
    pub target_window: Option<&'a str>,
    #[cfg(feature = "tmux_1_0")]
    pub template: Option<&'a str>,
}

impl<'a> ChooseClientBuilder<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    #[cfg(feature = "tmux_2_6")]
    pub fn without_preview(&mut self) -> &mut Self {
        self.without_preview = Some(true);
        self
    }

    #[cfg(feature = "tmux_3_1")]
    pub fn reverse_sort_order(&mut self) -> &mut Self {
        self.reverse_sort_order = Some(true);
        self
    }

    #[cfg(feature = "tmux_3_1")]
    pub fn zoom(&mut self) -> &mut Self {
        self.zoom = Some(true);
        self
    }

    #[cfg(feature = "tmux_1_7")]
    pub fn format(&mut self, format: &'a str) -> &mut Self {
        self.format = Some(format);
        self
    }

    #[cfg(feature = "tmux_2_6")]
    pub fn filter(&mut self, filter: &'a str) -> &mut Self {
        self.filter = Some(filter);
        self
    }

    #[cfg(feature = "tmux_2_6")]
    pub fn sort_order(&mut self, sort_order: &'a str) -> &mut Self {
        self.sort_order = Some(sort_order);
        self
    }

    #[cfg(feature = "tmux_2_6")]
    pub fn target_pane(&mut self, target_pane: &'a str) -> &mut Self {
        self.target_pane = Some(target_pane);
        self
    }

    #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_6")))]
    pub fn target_window(&mut self, target_pane: &'a str) -> &mut Self {
        self.target_window = Some(target_pane);
        self
    }

    #[cfg(feature = "tmux_1_0")]
    pub fn template(&mut self, template: &'a str) -> &mut Self {
        self.template = Some(template);
        self
    }

    pub fn build(&self) -> ChooseClient<'a> {
        ChooseClient {
            #[cfg(feature = "tmux_2_6")]
            without_preview: self.without_preview,
            #[cfg(feature = "tmux_3_1")]
            reverse_sort_order: self.reverse_sort_order,
            #[cfg(feature = "tmux_3_1")]
            zoom: self.zoom,
            #[cfg(feature = "tmux_1_7")]
            format: self.format,
            #[cfg(feature = "tmux_2_6")]
            filter: self.filter,
            #[cfg(feature = "tmux_2_6")]
            sort_order: self.sort_order,
            #[cfg(feature = "tmux_2_6")]
            target_pane: self.target_pane,
            #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_6")))]
            target_window: self.target_window,
            #[cfg(feature = "tmux_1_0")]
            template: self.template,
        }
    }
}

impl<'a> TmuxInterface<'a> {
    const CHOOSE_CLIENT: &'static str = "choose-client";

    /// Put a pane into client mode, allowing a client to be selected interactively from a list
    ///
    /// # Manual
    ///
    /// tmux ^3.1:
    /// ```text
    /// tmux choose-client [-NrZ] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
    /// ```
    ///
    /// tmux ^2.7:
    /// ```text
    /// tmux choose-client [-NZ] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
    /// ```
    ///
    /// tmux ^2.6:
    /// ```text
    /// tmux choose-client [-N] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
    /// ```
    ///
    /// tmux ^1.7:
    /// ```text
    /// tmux choose-client [-F format] [-t target-window] [template]
    /// ```
    ///
    /// tmux ^1.0:
    /// ```text
    /// tmux choose-client  [-t target-window] [template]
    /// ```
    pub fn choose_client(
        &mut self,
        choose_client: Option<&ChooseClient>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(choose_client) = choose_client {
            #[cfg(feature = "tmux_2_6")]
            if choose_client.without_preview.unwrap_or(false) {
                args.push(N_KEY);
            }
            #[cfg(feature = "tmux_3_1")]
            if choose_client.reverse_sort_order.unwrap_or(false) {
                args.push(r_KEY);
            }
            #[cfg(feature = "tmux_3_1")]
            if choose_client.zoom.unwrap_or(false) {
                args.push(Z_KEY);
            }
            #[cfg(feature = "tmux_1_7")]
            if let Some(format) = choose_client.format {
                args.extend_from_slice(&[F_KEY, &format])
            }
            #[cfg(feature = "tmux_2_6")]
            if let Some(filter) = choose_client.filter {
                args.extend_from_slice(&[f_KEY, &filter])
            }
            #[cfg(feature = "tmux_2_6")]
            if let Some(sort_order) = choose_client.sort_order {
                args.extend_from_slice(&[O_KEY, &sort_order])
            }
            #[cfg(feature = "tmux_2_6")]
            if let Some(target_pane) = choose_client.target_pane {
                args.extend_from_slice(&[t_KEY, &target_pane])
            }
            #[cfg(all(feature = "tmux_1_0", not(feature = "tmux_2_6")))]
            if let Some(target_window) = choose_client.target_window {
                args.extend_from_slice(&[t_KEY, &target_window])
            }
            #[cfg(feature = "tmux_1_0")]
            if let Some(template) = choose_client.template {
                args.push(&template)
            }
        }
        let output = self.command(TmuxInterface::CHOOSE_CLIENT, &args)?;
        Ok(output)
    }
}