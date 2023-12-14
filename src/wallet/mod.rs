pub mod error;
pub mod commands;

use std::{fs::OpenOptions, io::{Write, self, Read}};
use error::Error;

use crate::crypto;
use crate::transaction::{Transaction, TxOut};

type Result<T> = std::result::Result<T, Error>;

const PUBLIC_KEY_HASH_LENGTH: usize = 20;
const BASE58_DECODED_ADDRESS_LENGTH: usize = 25; // version byte + payload + checksum
const CHECK_SUM_LENGTH: usize = 4;


pub struct Wallet {
    default_private_key: Vec<u8>,
    pending_tx: Vec<Transaction>,
}

impl Wallet {

    pub fn new() -> Wallet {
        let pk = crypto::generate_signing_key();
        Wallet { default_private_key: pk, pending_tx: Vec::new() }
    }

    // CLI commands --------------------------

    fn write_private_key(&self, key: String) -> Result<()> {
        let mut f = OpenOptions::new()
            .write(true).create(true).append(true)
            .open("keys.txt")
            .map_err(Error::IoError)?;
        writeln!(f, "{}", key).map_err(Error::IoError)?;
        Ok(())
    }

    // TODO: get arguments in order to set compress and prefix
    pub fn get_new_address(&self) -> Result<()> {
        let private_key = crypto::generate_signing_key();
        match crypto::get_pub_key(&private_key, true) {
            Ok(pub_key) => {
                match self.write_private_key(hex::encode(&private_key)) {
                    Ok(_) => println!("{}", crypto::get_address(&pub_key, 0)),
                    Err(e) => eprintln!("{}", e)
                }
            },
            Err(e) => eprintln!("{}", e)
        }

        Ok(())
    }

    pub fn get_balance(&self) -> Result<()> {
        Ok(())
    }

    pub fn create_tx(&mut self, args: &Vec<String>) -> Result<()> {
        let data: Vec<(f32, String)> = Self::convert_vout_args_to_tuples(args)?;
        let mut vout: Vec<TxOut> = Vec::new();

        for d in data { vout.push(Self::create_vout(d)?); }
        self.pending_tx.push(Transaction::from_vout(vout));
        
        Ok(())
    }

    pub fn sign_tx(&mut self, args: &Vec<String>) -> Result<()> {
        let index;
        match args.len() {
            0 => {
                match self.get_pending_transaction_index()? {
                    Some(i) => index = i,
                    None => return Ok(())
                }
            }
            1 => {
                match self.parse_and_check_pending_tx_index(args[0].clone())? {
                    Some(i) => index = i,
                    None => return Ok(())
                }
            }
            _ => return Err(Error::UnvalidArgsCountError(String::from("signtx"), 1))
        };
        self.pending_tx[index].sign(&self.default_private_key);
        Ok(())
    }

    pub fn send_tx(&self) -> Result<()> {
        Ok(())
    }

    pub fn show_pending_tx(&self) -> Result<()> {
        self.print_pending_transactions();
        Ok(())
    }

    pub fn list_unspent(&self) -> Result<()> {
        Ok(())
    }
}

// COMMAND: createtx --------------------------------------------------------------------------------
impl Wallet {
    fn convert_vout_args_to_tuples(args: &Vec<String>) -> Result<Vec<(f32, String)>> {
        let mut new_recipient = true;
        let mut current_recipient: (f32, String) = (0.0, String::new());
        let mut data: Vec<(f32, String)> = Vec::new();

        if args.len() == 0 || args.len() % 2 != 0 {
            return Err(Error::NotEnoughArgsError(String::from("amount/address missing, or arguments were not correctly formated.")))
        }

        for arg in args {
            if new_recipient {
                current_recipient.0 = arg.parse().map_err(Error::ParseFloatError)?
            } else {
                current_recipient.1.push_str(&arg);
                data.push(current_recipient.clone());
                current_recipient.1.clear();
            }
            new_recipient = !new_recipient;
        }
        
        Ok(data)
    }

    fn create_vout(data: (f32, String)) -> Result<TxOut> {
        let slice = bs58::decode(data.1).into_vec().map_err(Error::Base58Error)?;
        match slice.len() {
            BASE58_DECODED_ADDRESS_LENGTH => {
                let public_key_hash = Self::get_public_key_hash(&slice)?;
                return Ok(TxOut::new(data.0, public_key_hash));
            },
            _ => Err(Error::UnvalidAddressError(String::from("Invalid Base58 decoded address length")))
        }
    }

    fn get_public_key_hash(slice: &[u8]) -> Result<[u8; PUBLIC_KEY_HASH_LENGTH]> {
        let checksum = Self::get_checksum(slice)?;
        let checksum_index = slice.len() - CHECK_SUM_LENGTH;

        if crypto::get_checksum(&slice[..checksum_index]) == checksum {
            let payload = &slice[1..checksum_index];
            let public_key_hash = Self::get_public_key_hash_from_slice(payload)?;
            return Ok(public_key_hash);
        }

        return Err(Error::UnvalidCheckSumError);
    }

    fn get_checksum(slice: &[u8]) -> Result<[u8; CHECK_SUM_LENGTH]> {
        if slice.len() > CHECK_SUM_LENGTH {
            let check_sum_index = slice.len() - CHECK_SUM_LENGTH;
            let mut checksum = [0u8; CHECK_SUM_LENGTH];
            checksum.copy_from_slice(&slice[check_sum_index..]);
            return Ok(checksum);
        }
        Err(Error::UnvalidAddressError(format!("Public key hash + version byte length is less than {}", CHECK_SUM_LENGTH)))
    }

    fn get_public_key_hash_from_slice(slice: &[u8]) -> Result<[u8; PUBLIC_KEY_HASH_LENGTH]> {
        match slice.len() {
            PUBLIC_KEY_HASH_LENGTH => {
                let mut public_key_hash = [0u8; PUBLIC_KEY_HASH_LENGTH];
                public_key_hash.copy_from_slice(slice);
                return Ok(public_key_hash);
            }
            _ => return Err(Error::UnvalidAddressError(String::from("Public key hash length converted from the address you provided is not valid")))
        }
    }
}


// COMMAND: signtx --------------------------------------------------------------------------------
impl Wallet {

}

// Methods used by multiple commands --------------------------------------------------------------------------------
impl Wallet {


    // TODO: find a way to parse buffer[0; 1] (Ascii code) to integer
    fn get_pending_transaction_index(&self) -> Result<Option<usize>> {
        self.print_pending_transactions();
        loop {
            let mut buffer = [0; 1];
            io::stdin().read_exact(&mut buffer).map_err(Error::IoError)?;

            match String::from_utf8(buffer.to_vec()) {
                Ok(input) => {
                    if input == "q" { break; }
                    if let Some(index) = self.parse_and_check_pending_tx_index(input)? {
                        return Ok(Some(index));
                    }
                },
                Err(e) => eprint!("Not a valid index: {}", e)
            }
        }
        Ok(None)
    }


    fn print_pending_transactions(&self) {
        for i in 0..self.pending_tx.len() {
            println!("Pending tx: {}", i);
            println!("{}", self.pending_tx[i]);
        }   
    }

    // Think about converting to generic function
    fn parse_and_check_pending_tx_index(&self, input: String) -> Result<Option<usize>> {
        let index: usize = input.parse().map_err(Error::ParseUsizeError)?;
        if index < self.pending_tx.len() {
            return Ok(Some(index));
        }
        eprintln!("Index out of range");
        Ok(None)
    }
}