/// All functions from man tmux "Clients and Sessions" listed below
/// [man tmux](http://man7.org/linux/man-pages/man1/tmux.1.html#CLIENTS_AND_SESSIONS)
#[cfg(feature = "tmux_1_0")]
pub mod attach_session;
#[cfg(feature = "tmux_1_0")]
pub mod detach_client;
#[cfg(feature = "tmux_1_0")]
pub mod has_session;
#[cfg(feature = "tmux_1_0")]
pub mod kill_server;
#[cfg(feature = "tmux_1_0")]
pub mod kill_session;
#[cfg(feature = "tmux_1_0")]
pub mod list_clients;
#[cfg(feature = "tmux_1_0")]
pub mod list_commands;
#[cfg(feature = "tmux_1_0")]
pub mod list_sessions;
#[cfg(feature = "tmux_1_1")]
pub mod lock_client;
#[cfg(feature = "tmux_1_1")]
pub mod lock_session;
#[cfg(feature = "tmux_1_0")]
pub mod new_session;
#[cfg(feature = "tmux_1_0")]
pub mod refresh_client;
#[cfg(feature = "tmux_1_0")]
pub mod rename_session;
#[cfg(feature = "tmux_1_2")]
pub mod show_messages;
#[cfg(feature = "tmux_1_0")]
pub mod source_file;
#[cfg(feature = "tmux_1_0")]
pub mod start_server;
#[cfg(feature = "tmux_1_0")]
pub mod suspend_client;
#[cfg(feature = "tmux_1_0")]
pub mod switch_client;

#[cfg(feature = "tmux_1_0")]
pub mod attach_session_tests;
#[cfg(feature = "tmux_1_0")]
pub mod detach_client_tests;
#[cfg(feature = "tmux_1_0")]
pub mod has_session_tests;
#[cfg(feature = "tmux_1_0")]
pub mod kill_server_tests;
#[cfg(feature = "tmux_1_0")]
pub mod kill_session_tests;
#[cfg(feature = "tmux_1_0")]
pub mod list_clients_tests;
#[cfg(feature = "tmux_1_0")]
pub mod list_commands_tests;
#[cfg(feature = "tmux_1_0")]
pub mod list_sessions_tests;
#[cfg(feature = "tmux_1_1")]
pub mod lock_client_tests;
#[cfg(feature = "tmux_1_1")]
pub mod lock_session_tests;
#[cfg(feature = "tmux_1_0")]
pub mod new_session_tests;
#[cfg(feature = "tmux_1_0")]
pub mod refresh_client_tests;
#[cfg(feature = "tmux_1_0")]
pub mod rename_session_tests;
#[cfg(feature = "tmux_1_2")]
pub mod show_messages_tests;
#[cfg(feature = "tmux_1_0")]
pub mod source_file_tests;
#[cfg(feature = "tmux_1_0")]
pub mod start_server_tests;
#[cfg(feature = "tmux_1_0")]
pub mod suspend_client_tests;
#[cfg(feature = "tmux_1_0")]
pub mod switch_client_tests;
