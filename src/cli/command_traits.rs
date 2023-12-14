pub enum Command {
    GETNEWADDRESS,
    GETBALANCE,

    CREATETX,
    SIGNTX,
    SENDTX,
    SHOWPENDING,

    LISTUNSPENT,
}
pub trait CommandParam {
    const COMMAND: Command;
    const MIN_ARG_LENGTH: usize;
    const MAX_ARG_LENGTH: Option<usize>;
    const VALID_FLAGS: Vec<&'static str>;
}

pub trait Help {
    const HELP_DESC: &'static str;
}