use serde::{Serialize, Deserialize};
use crate::crypto;

#[derive(Serialize, Deserialize)]
struct TxIn {
    signature: Vec<u8>,
    txid: [u8; crypto::SHA256_DIGEST_LENGTH],         // references which transaction to use
    n: u8,                  // references which txout to use
}

#[derive(Serialize, Deserialize)]
pub struct TxOut {
    value: f32,
    public_key_hash: [u8; 20]
}

impl TxOut {
    pub fn new(value: f32, public_key_hash: [u8; 20]) -> TxOut {
        TxOut{ value, public_key_hash}
    }
}

#[derive(Serialize, Deserialize)]
pub struct Transaction {
    vin: Vec<TxIn>,
    vout: Vec<TxOut>
}

impl TxIn {
    fn serialize(&self) -> Vec<u8> {
        let mut buffer: Vec<u8> = Vec::new();
        buffer.extend_from_slice(&self.txid);
        buffer.push(self.n);
        buffer
    }
}

impl TxOut {
    fn serialize(&self) -> Vec<u8> {
        let mut buffer: Vec<u8> = Vec::new();
        buffer.extend_from_slice(&self.value.to_ne_bytes());
        buffer.extend_from_slice(&self.public_key_hash);
        buffer
    }
}

impl Transaction {
    pub fn from_vout(vout: Vec<TxOut>) -> Transaction {
        Transaction { vin: Vec::new(), vout }
    }

    pub fn serialize_and_hash(&self) -> [u8; crypto::SHA256_DIGEST_LENGTH] {
        let mut hash = [0u8; crypto::SHA256_DIGEST_LENGTH];
        let mut buffer: Vec<u8> = Vec::new();
        self.vin.iter().for_each(|txin| buffer.append(&mut txin.serialize()) );
        self.vout.iter().for_each(|txout| buffer.append(&mut txout.serialize()) );
        crypto::sha256(&buffer)
    }

    pub fn sign(&mut self, private_key: &[u8]) -> bool {
        // For now, this only works for a signgle signer per transaction
        if let Ok(signature) = crypto::sign(private_key, &self.serialize_and_hash()) {
            self.vin.iter_mut().for_each(|txin| txin.signature = signature.clone());
            return true;
        }
        false
    }
}

impl std::fmt::Display for TxIn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tab = "    ";
        writeln!(f, "{{")?;
        writeln!(f, "{}{}Signature: {},", tab, tab, hex::encode(&self.signature))?;
        writeln!(f, "{}{}TxID: {},", tab, tab, hex::encode(self.txid))?;
        writeln!(f, "{}{}n: {},", tab, tab, self.n)?;
        writeln!(f, "{}}}", tab)
    }
}

impl std::fmt::Display for TxOut {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tab = "    ";
        writeln!(f, "{{")?;
        writeln!(f, "{}{}Value: {}", tab, tab, self.value)?;
        writeln!(f, "{}{}PubKeyHash: {}", tab, tab, hex::encode(self.public_key_hash))?;
        writeln!(f, "{}}}", tab)
        
    }
}

impl std::fmt::Display for Transaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tab = "    ";
        writeln!(f, "{{")?;
        writeln!(f, "{}vin {{", tab)?;
        for txin in &self.vin {
            writeln!(f, "{{")?;
            writeln!(f, "{}{}Signature: {},", tab, tab, hex::encode(&txin.signature))?;
            writeln!(f, "{}{}TxID: {},", tab, tab, hex::encode(txin.txid))?;
            writeln!(f, "{}{}n: {},", tab, tab, txin.n)?;
            writeln!(f, "{}}}", tab)?;
        }
        writeln!(f, "{}}}", tab)?;

        writeln!(f, "{}vout {{", tab)?;
        for txout in &self.vout {
            writeln!(f, "{}{}{{", tab, tab)?;
            writeln!(f, "{}{}{}Value: {}", tab, tab, tab, txout.value)?;
            writeln!(f, "{}{}{}PubKeyHash: {}", tab, tab, tab, hex::encode(txout.public_key_hash))?;
            writeln!(f, "{}{}}}", tab, tab)?;
        }
        writeln!(f, "{}}}", tab)?;
        writeln!(f, "}}")
    }
}