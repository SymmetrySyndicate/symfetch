//! symfetch
//!
//! ```text
//!   ,-~~-.___.                                                | sauravmaheshkar@artemis.local (06/13/25 14:37)
//!  / |  '     \         It was a dark and stormy night....    |
//! (  )         0                                              | OS: macOS 15.5 Sequoia 24.5.0
//!  \_/-, ,----'                                               | Uptime: 9d 23h 12m
//!     ====             //                                     | Shell: zsh
//!    /  \-'~;      /~~~(O)                                    | Displays: 1
//!   /  __/~|     /       |                                    | Terminal: xterm-256color
//! =(  _____|   (_________|   W<                               | CPU: Apple M1 (8 cores)
//!                                                             | Memory: 5850MB / 8192MB
//!                                                             | Storage: 266GB / 456GB
//! ```
//!
//! ## Usage
//!
//! You can run the executable by simply calling the symfetch cmd.
//!
//! ```bash
//! symfetch
//! ```
//!
//! `symfetch` by default looks for a config file in `~/.config/symfetch.toml`,
//! if you want to place your config file somewhere else you can pass that path using the `-c` or `--config` argument.
//!
//! For example,
//!
//! ```bash
//! symfetch -c ~/symfetch.toml
//! ```
//!
//! ## Configuration
//!
//! We use toml for configuring `symfetch`. The main configuration you need to specify is whether to use
//! an ASCII art or some image as the graphic. Both these option requires you to specify a path value with a
//! `[ascii]` or a `[image]` table. For instance
//!
//! ASCII only configuration file
//!
//! ```toml
//! [ascii]
//! path="ascii"
//! ```
//!
//! Image only configuration file
//!
//! ```toml
//! [image]
//! path="image.png"
//! # Optional: as_ascii = false
//! # Optional: height = 20
//! # Optional: width = 160
//! # Optional: colored = true
//! ```
//!
//! ## Brought to you by
//!
//! ![](https://github.com/SymmetrySyndicate/.github/blob/main/assets/banner/twitter_banner.png?raw=true)
pub mod config_handler;
pub mod data;
pub mod system_info;
pub mod util;
