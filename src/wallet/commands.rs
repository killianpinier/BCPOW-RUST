use crate::cli::command_traits::{Command, CommandParam};

pub struct GetNewAddress;
pub struct GetBalance;
pub struct CreateTx;
pub struct SignTx;
pub struct SendTx;
pub struct ShowPending;
pub struct ListUnspent;

// impl CommandParam for GetNewAddress {
//     const COMMAND: Command = Command::GETNEWADDRESS;
//     const MIN_ARG_LENGTH: usize = 0;
//     const MAX_ARG_LENGTH: Option<usize> = Some(0);
//     const VALID_FLAGS: Vec<&'static str> = vec!["h", "help"];
// }

// impl CommandParam for GetBalance {
//     const COMMAND: Command = Command::GETBALANCE;
//     const MIN_ARG_LENGTH: usize = 0;
//     const MAX_ARG_LENGTH: Option<usize> = Some(0);
//     const VALID_FLAGS: Vec<&'static str> = vec!["h", "help"];
// }

// impl CommandParam for CreateTx {
//     const COMMAND: Command = Command::CREATETX;
//     const MIN_ARG_LENGTH: usize = 2;
//     const MAX_ARG_LENGTH: Option<usize> = None;
//     const VALID_FLAGS: Vec<&'static str> = vec!["h", "help"];
// }

// impl CommandParam for SignTx {
//     const COMMAND: Command = Command::SIGNTX;
//     const MIN_ARG_LENGTH: usize = 0;
//     const MAX_ARG_LENGTH: Option<usize> = Some(1);
//     const VALID_FLAGS: Vec<&'static str> = vec!["h", "help"];
// }

// impl CommandParam for SendTx {
//     const COMMAND: Command = Command::SENDTX;
//     const MIN_ARG_LENGTH: usize = 0;
//     const MAX_ARG_LENGTH: Option<usize> = Some(1);
//     const VALID_FLAGS: Vec<&'static str> = vec!["h", "help"];
// }

// impl CommandParam for ShowPending {
//     const COMMAND: Command = Command::SHOWPENDING;
//     const MIN_ARG_LENGTH: usize = 0;
//     const MAX_ARG_LENGTH: Option<usize> = Some(0);
//     const VALID_FLAGS: Vec<&'static str> = vec!["h", "help"];
// }

// impl CommandParam for ListUnspent {
//     const COMMAND: Command = Command::LISTUNSPENT;
//     const MIN_ARG_LENGTH: usize = 0;
//     const MAX_ARG_LENGTH: Option<usize> = Some(0);
//     const VALID_FLAGS: Vec<&'static str> = vec!["h", "help"];
// }