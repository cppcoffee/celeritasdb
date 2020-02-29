use crate::qpaxos::{Instance, InstanceID};
use crate::replica::ReplicaID;
use crate::tokey::ToKey;
use std::fmt::LowerHex;

// required by encode/decode
use prost::Message;

use super::Error;
use super::InstanceIter;

pub struct BaseIter<'a> {
    pub cursor: Vec<u8>,
    pub include: bool,
    pub engine: &'a dyn Base,
}

impl<'a> Iterator for BaseIter<'a> {
    type Item = (Vec<u8>, Vec<u8>);

    // TODO add unittest.
    fn next(&mut self) -> Option<Self::Item> {
        let r = self.engine.next_kv(&self.cursor, self.include);
        self.include = false;
        match r {
            Some(kv) => {
                self.cursor = kv.0.clone();
                Some(kv)
            }
            None => None,
        }
    }
}

/// Base offer basic key-value access
pub trait Base {
    /// set a new key-value
    fn set_kv(&mut self, key: Vec<u8>, value: Vec<u8>) -> Result<(), Error>;

    /// get an existing value with key
    fn get_kv(&self, key: &Vec<u8>) -> Result<Vec<u8>, Error>;

    /// next_kv returns a key-value pair greater than the given one(include=false),
    /// or greater or equal the given one(include=true)
    fn next_kv(&self, key: &Vec<u8>, include: bool) -> Option<(Vec<u8>, Vec<u8>)>;

    fn get_iter(&self, key: Vec<u8>, include: bool) -> BaseIter;
}

/// InstanceEngine offer functions to operate snapshot instances
pub trait InstanceEngine: StatusEngine {
    /// set a new instance
    fn set_instance(&mut self, iid: InstanceID, inst: Instance) -> Result<(), Error>;
    /// update an existing instance with instance id
    fn update_instance(&mut self, iid: InstanceID, inst: Instance) -> Result<(), Error>;
    /// get an instance with instance id
    fn get_instance(&self, iid: &InstanceID) -> Result<Instance, Error>;
    /// get an iterator to scan all instances with a leader replica id
    fn get_instance_iter(&self, rid: ReplicaID) -> InstanceIter;
}

/// StatusEngine offer functions to operate snapshot status
pub trait StatusEngine: TxEngine + ColumnedEngine {
    /// get current maximum instance id with a leader replica
    fn get_max_instance_id(&self, col_id: Self::ColumnId) -> Result<Self::ObjId, Error> {
        self.get_ref("max", rid)
    }

    /// get executed maximum continuous instance id with a leader replica
    fn get_max_exec_instance_id(&self, rid: Self::ColumnId) -> Result<Self::ObjId, Error> {
        self.get_ref("exec", rid)
    }

    fn set_instance_id(&mut self, key: Vec<u8>, iid: InstanceID) -> Result<(), Error> {
        let mut value = vec![];
        iid.encode(&mut value).unwrap();
        self.set_kv(key, value)
    }
}

/// TxEngine offer a transactional operation on a storage.
pub trait TxEngine {
    /// start a transaction
    fn trans_begin(&mut self);
    /// commit a transaction
    fn trans_commit(&mut self) -> Result<(), Error>;
    /// rollback a transaction
    fn trans_rollback(&mut self) -> Result<(), Error>;
    /// get a key to set exclusively, must be called in an transaction
    fn get_kv_for_update(&self, key: &Vec<u8>) -> Result<Vec<u8>, Error>;
}

/// ObjectEngine wraps bytes based storage engine into an object based engine.
/// Structured object can be stored and retreived with similar APIs.
/// An object is serialized into bytes with protobuf engine prost.
///
/// TODO example
pub trait ObjectEngine: Base {
    /// ObjId defines the type of object id.
    /// It must be able to convert to a key in order to store an object.
    /// Alsot it needs to be serialized as Message in order to be stored as an object too.
    type ObjId: ToKey + Message + std::default::Default;

    /// Obj defines the type of an object.
    type Obj: Message + std::default::Default;

    fn set_obj(&mut self, item_id: Self::ObjId, item: &Self::Obj) -> Result<(), Error> {
        let key = item_id.to_key();
        let value = self.encode_obj(item)?;

        self.set_kv(key, value)
    }

    fn get_obj(&self, item_id: &Self::ObjId) -> Result<Self::Obj, Error> {
        let key = item_id.to_key();
        let val_bytes = self.get_kv(&key)?;

        let itm = self.decode_obj(&val_bytes)?;
        Ok(itm)
    }

    fn encode_obj(&self, itm: &Self::Obj) -> Result<Vec<u8>, Error> {
        let mut value = vec![];
        itm.encode(&mut value).unwrap();
        Ok(value)
    }

    fn decode_obj(&self, bs: &Vec<u8>) -> Result<Self::Obj, Error> {
        match Self::Obj::decode(bs.as_slice()) {
            Ok(v) => Ok(v),
            Err(_) => Err(Error::DBError {
                msg: "parse instance id error".to_string(),
            }),
        }
    }
}

/// ColumnedEngine organizes object in columns.
/// Because the underlying storage is a simple object store,
/// it introduces ColumnId to classify objects.
/// And also it provides APIs to track objects in different columns.
///
/// set_ref(type, col_id, obj_id) to store a column reference of `type` to be `obj_id`.
///
/// E.g.: `set_ref("max", 1, (1, 2))` to set the "max" object in column 1 to be object with object-id
/// (1, 2)
///
/// A User should implement make_ref_key() to make reference keys.
pub trait ColumnedEngine: ObjectEngine {
    type ColumnId;

    fn make_ref_key(&self, typ: &str, col_id: Self::ColumnId) -> Vec<u8>;

    fn set_ref(
        &mut self,
        typ: &str,
        col_id: Self::ColumnId,
        item_id: Self::ObjId,
    ) -> Result<(), Error> {
        let key = self.make_ref_key(typ, col_id);

        let mut value = vec![];
        item_id.encode(&mut value).unwrap();

        self.set_kv(key, value)
    }

    fn get_ref(&self, typ: &str, col_id: Self::ColumnId) -> Result<Self::ObjId, Error> {
        let key = self.make_ref_key(typ, col_id);
        let val_bytes = self.get_kv(&key)?;

        match Self::ObjId::decode(val_bytes.as_slice()) {
            Ok(v) => Ok(v),
            Err(_) => Err(Error::DBError {
                msg: "parse instance id error".to_string(),
            }),
        }
    }
}
