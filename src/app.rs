use crate::cli::{Cli, Instruction};
use crate::wallet;
use crate::miner;
use std::fmt::Display;

#[derive(Debug)]
enum AppError {
    // UnrecognizedFlagError(String),
    // UnvalidArgLengthError(String),
    WalletError(wallet::error::Error),
}

impl std::error::Error for AppError {}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // AppError::UnrecognizedFlagError(flag)  => write!(f, "Unrecognized flag: {}", flag),
            // AppError::UnvalidArgLengthError(e)     => write!(f, "Unvalid number of argument: {}", e),
            AppError::WalletError(e)                => write!(f, "{}", e)
        }
    }
}

impl From<wallet::error::Error> for AppError {
    fn from(err: wallet::error::Error) -> Self {
        AppError::WalletError(err)
    }
}


pub struct App {
    cli: Cli,
    wallet: wallet::Wallet,
    miner: miner::Miner
}

impl App {
    pub fn new(app_name: String) -> App {
        let miner = miner::Miner::new();
        let wallet = wallet::Wallet::new();
        App { cli: Cli::new(app_name), wallet, miner }
    }

    pub fn run(&mut self) {
        let mut stop = false;
        while !stop {
            match self.cli.get_instruction() {
                Ok(Some(instruction)) => {
                    match self.parse_command(instruction) {
                        Ok(s) => stop = s,
                        Err(e) => eprintln!("{}", e)
                    }
                }
                Ok(None) => {}
                Err(e) => eprint!("{}", e)
            }
        };
    }

    fn parse_command(&mut self, instruction: Instruction) -> Result<bool, AppError> {
        match instruction.get_command() {
            // "getnewaddress" => self.execute::<commands::GetNewAddress, _, _>(|| self.wallet.get_new_address(), &instruction),
            "getnewaddress" => self.wallet.get_new_address().map(|_| false),
            "getbalance"    => self.wallet.get_balance().map(|_| false),
            "createtx"      => self.wallet.create_tx(instruction.get_arguments()).map(|_| false),
            "signtx"        => self.wallet.sign_tx(instruction.get_arguments()).map(|_| false),
            "sendtx"        => self.wallet.send_tx().map(|_| false),
            "showpending"   => self.wallet.show_pending_tx().map(|_| false),

            "listunspent"   => self.wallet.list_unspent().map(|_| false),

            "exit"          => Ok(true),

            _ => Ok(false)
        }.map_err(Into::into)
    }

    // fn execute<T: CommandParam, F, E>(&self, execute_method: F, instruction: &Instruction) -> Result<bool, AppError>
    // where F: FnOnce() -> Result<(), E>, E: Into<AppError>  {
    //     Self::check_instruction::<T>(instruction)?;
    //     execute_method().map(|_| false).map_err(Into::into)
    // }
}


// impl App {
//     fn check_instruction<T: CommandParam>(instruction: &Instruction) -> Result<(), AppError> {
//         Self::check_args_count::<T>(instruction.get_arguments().len())?;
//         Self::check_flags::<T>(instruction.get_flags())?;
//         Ok(())
//     }

//     fn check_args_count<T: CommandParam>(args_len: usize) -> Result<(), AppError> {
//         if args_len < T::MIN_ARG_LENGTH {
//             return Err(AppError::UnvalidArgLengthError(format!("{} arguments are required", T::MIN_ARG_LENGTH)))
//         }
//         if let Some(max) = T::MAX_ARG_LENGTH {
//             if args_len > max {
//                 return Err(AppError::UnvalidArgLengthError(String::from("")))
//             }
//         }
//         Ok(())
//     }

//     // How to convert automatically to &str without: let flag: &str = flag;
//     fn check_flags<T: CommandParam>(flags: &Vec<String>) -> Result<(), AppError> {
//         for flag in flags {
//             let flag: &str = flag;
//             if !T::VALID_FLAGS.contains(&flag) {
//                 return Err(AppError::UnrecognizedFlagError(String::from(flag)))
//             }
//         }
//         Ok(())
//     }
// }