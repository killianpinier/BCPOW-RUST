use std::{rc::Rc, marker::PhantomData};

use rocksdb::{DB, Options, ColumnFamilyDescriptor, ColumnFamily};
use serde::{Serialize, de::DeserializeOwned};

#[derive(Debug)]
pub enum Error {
    DbError(rocksdb::Error),
    BincodeError(bincode::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::DbError(io_error) => write!(f, "{}", io_error),
            Error::BincodeError(io_error) => write!(f, "{}", io_error),
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;

pub struct Rocks {
    db: rocksdb::DB,
}

impl Rocks {
    pub fn open(path: &str, cfs: Vec<ColumnFamilyDescriptor>) -> Result<Self> {
        let mut db_opts = Options::default();
        db_opts.create_if_missing(true);
        db_opts.create_missing_column_families(true);

        let db = DB::open_cf_descriptors(&db_opts, path, cfs).map_err(Error::DbError)?;

        Ok( Rocks { db } )
    }

    pub fn cf_handle(&self, cf: &str) -> &ColumnFamily {
        self.db.cf_handle(cf).unwrap()
    }

    fn put_cf(&self, cf: &ColumnFamily, key: &[u8], value: &[u8]) -> Result<()> {
        self.db.put_cf(cf, key, value).map_err(Error::DbError)
    }

    fn get_cf(&self, cf: &ColumnFamily, key: &[u8]) -> Result<Option<Vec<u8>>> {
        self.db.get_cf(cf, key).map_err(Error::DbError)
    }

    fn delete_cf(&self, cf: &ColumnFamily, key: &[u8]) -> Result<()> {
        self.db.delete_cf(cf, key).map_err(Error::DbError)
    }
}




pub struct LedgerColumn<T: ColumnName + ColumnType> {
    db: Rc<Rocks>,
    column: PhantomData<T>,
}

impl<T: ColumnName + ColumnType> LedgerColumn<T> {
    pub fn new(db: Rc<Rocks>) -> LedgerColumn<T> {
        LedgerColumn { db, column: PhantomData }
    }

    fn get_handle(&self) -> &ColumnFamily {
        self.db.cf_handle(T::NAME)
    }

    pub fn put(&self, key: &[u8], value: &T::Type) -> Result<()> {
        let serialized_value = bincode::serialize(value).map_err(Error::BincodeError)?;
        self.db.put_cf(self.get_handle(), key, serialized_value.as_slice())
    }

    pub fn get(&self, key: &[u8]) -> Result<Option<T::Type>> {
        if let Some(bytes) = self.db.get_cf(self.get_handle(), key)? {
            let value = bincode::deserialize(bytes.as_slice()).map_err(Error::BincodeError)?;
            return Ok(Some(value));
        }
        Ok(None)
    }
}


pub trait ColumnName {
    const NAME: &'static str;
}

pub trait ColumnType {
    type Type: Serialize + DeserializeOwned;
}