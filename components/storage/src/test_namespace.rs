use crate::test_engine::*;
use crate::DBColumnFamily;
use crate::MemEngine;
use crate::WithNs;
use std::sync::Arc;

use crate::traits::Base;
use crate::WriteEntry;

use crate::NameSpace;

#[test]
fn test_namespace() {
    assert_eq!("5/foo".as_bytes().to_vec(), 5i64.wrap_ns("foo".as_bytes()));
    assert_eq!(
        "bar/foo".as_bytes().to_vec(),
        "bar".wrap_ns("foo".as_bytes())
    );

    assert_eq!(None, 5i64.unwrap_ns("6/foo".as_bytes()));
    assert_eq!(
        Some("foo".as_bytes().to_vec()),
        5i64.unwrap_ns("5/foo".as_bytes())
    );
}

#[test]
fn test_withns() {
    {
        let eng = MemEngine::new().unwrap();
        let eng = Arc::new(eng);
        let w = WithNs::new(5, eng);
        test_base_trait(&w);
    }

    {
        let eng = MemEngine::new().unwrap();
        let eng = Arc::new(eng);
        let w = WithNs::new(5, eng);
        test_kv_trait(&w);
    }

    {
        let eng = MemEngine::new().unwrap();
        let eng = Arc::new(eng);
        let w = WithNs::new(5, eng);
        test_columned_trait(&w);
    }

    {
        let eng = MemEngine::new().unwrap();
        let eng = Arc::new(eng);
        let w = WithNs::new(5, eng);
        test_instance_trait(&w);
    }
}

#[test]
fn test_withns_no_overriding() {
    let eng = MemEngine::new().unwrap();
    let eng = Arc::new(eng);
    let w1 = WithNs::new(1, eng.clone());
    let w2 = WithNs::new(2, eng.clone());

    let k = "foo".as_bytes().to_vec();
    let v1 = "111".as_bytes().to_vec();
    let v2 = "222".as_bytes().to_vec();

    {
        // no overriding for get/set

        w1.set(DBColumnFamily::Status, &k, &v1).unwrap();
        w2.set(DBColumnFamily::Status, &k, &v2).unwrap();

        let r = w1.get(DBColumnFamily::Status, &k).unwrap();
        assert_eq!(v1, r.unwrap());

        let r = w2.get(DBColumnFamily::Status, &k).unwrap();
        assert_eq!(v2, r.unwrap());
    }

    {
        // next/prev is bounded by namespace

        let r = w1.next(DBColumnFamily::Status, &k, true);
        assert_eq!((k.clone(), v1.clone()), r.unwrap());

        let r = w1.next(DBColumnFamily::Status, &k, false);
        assert!(r.is_none(), "next should not get k/v from other namespace");

        let r = w2.prev(DBColumnFamily::Status, &k, false);
        assert!(r.is_none(), "prev should not get k/v from other namespace");
    }

    {
        // write_batch should not override

        let k1 = "k1".as_bytes().to_vec();
        let k2 = "k2".as_bytes().to_vec();

        let batch = vec![
            WriteEntry::Set(DBColumnFamily::Default, k1.clone(), v1.clone()),
            WriteEntry::Set(DBColumnFamily::Status, k2.clone(), v1.clone()),
        ];

        w1.write_batch(&batch).unwrap();

        let r = w1.get(DBColumnFamily::Default, &k1).unwrap();
        assert_eq!(Some(v1), r);

        let r = w2.get(DBColumnFamily::Default, &k1).unwrap();
        assert!(r.is_none());

        let r = w2.get(DBColumnFamily::Status, &k2).unwrap();
        assert!(r.is_none());
    }
}
