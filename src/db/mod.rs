mod rocks;

use rocks::{Rocks, LedgerColumn, ColumnName, ColumnType, Result};
use crate::block::Block;
use crate::transaction::TxOut;

use rocksdb::{Options, ColumnFamilyDescriptor};
use std::rc::Rc;



pub struct Database {
    db: Rc<Rocks>,
    block_cf: LedgerColumn<columns::Block>,
    utxo_cf: LedgerColumn<columns::Utxo>
}

impl Database {
    pub fn open(path: &str) -> Result<Self> {
        let cf_descriptors = Self::get_cf_descriptors();

        let db = Rc::new(Rocks::open(path, cf_descriptors)?);
        let block_cf = LedgerColumn::new(Rc::clone(&db));
        let utxo_cf = LedgerColumn::new(Rc::clone(&db));

        Ok(Database { db, block_cf, utxo_cf })
    }

    fn get_cf_descriptors() -> Vec<ColumnFamilyDescriptor> {
        vec![
            ColumnFamilyDescriptor::new(columns::BLOCK_CF, Options::default()),
            ColumnFamilyDescriptor::new(columns::UTXO_CF, Options::default()),
        ]
    }

    pub fn get_block(&self, hash: &[u8; 32]) -> Result<Option<Block>> {
        self.block_cf.get(hash)
    }

    pub fn put_block(&self, block: &Block) -> Result<()> {
        self.block_cf.put(block.get_hash(), block)
    }

}


pub mod columns {
    pub const BLOCK_CF: &str = "block";
    pub struct Block;

    pub const UTXO_CF: &str = "utxo";
    pub struct Utxo;
}

impl ColumnName for columns::Block {
    const NAME: &'static str = columns::BLOCK_CF;
}

impl ColumnType for columns::Block {
    type Type = Block;
}

impl ColumnName for columns::Utxo {
    const NAME: &'static str = columns::UTXO_CF;
}

impl ColumnType for columns::Utxo {
    type Type = TxOut;
}


#[cfg(test)]
mod tests {
    use crate::block::Block;

    use super::Database;

    //#[test]
    fn add_block_test() {
        let db = Database::open("db-test").unwrap();
        let block = Block::new();

        db.put_block(&block).unwrap();
        let block_db = db.get_block(block.get_hash()).unwrap().unwrap();

        assert_eq!(*block.get_hash(), *block_db.get_hash())
    }

    //#[test]
    fn retrieve_block() {
        let db = Database::open("db-test").unwrap();
        let hash = [3; 32];
        let block_db = db.get_block(&hash).unwrap().unwrap();

        assert_eq!(block_db.get_index(), 1234);
    }
}