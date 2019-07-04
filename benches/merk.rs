#![feature(test)]

extern crate test;
extern crate rand;

use merk::*;
use merk::store::temporarydb::TemporaryDB;
use rand::prelude::*;

#[bench]
fn bench_put_insert_random(b: &mut test::Bencher) {
    let mut db = TemporaryDB::new();
    let mut merk = Merk::new(&mut db).unwrap();

    let value = [123; 40];

    for i in 0..250 {
        let mut keys = vec![];

        for j in 0..4_000 {
            let n = (i * 4_000 as u128) + (j as u128);
            keys.push(n.to_be_bytes());
        }

        let mut batch: Vec<TreeBatchEntry> = vec![];
        for key in keys.iter() {
            batch.push((&key[..], TreeOp::Put(&value)));
        }

        merk.apply_unchecked(&batch).unwrap();
    }

    let mut i = 0;
    b.iter(|| {
        let mut keys = vec![];
        for j in 0..4_000 {
            let n = i as u128 + (j * 100) as u128;
            keys.push(n.to_be_bytes());
        }

        let value = [123 as u8; 40];

        let mut batch: Vec<TreeBatchEntry> = vec![];
        for key in keys.iter() {
            batch.push((&key[..], TreeOp::Put(&value)));
        }

        merk.apply_unchecked(&batch).unwrap();

        i += 1;
    });

    println!("final tree size: {}", i * 4_000);
}

#[bench]
fn bench_put_update_random(b: &mut test::Bencher) {
    //let mut merk = Merk::open("./test_merk_bench_put_update_random.db").unwrap();
    let mut db = TemporaryDB::new();
    let mut merk = Merk::new(&mut db).unwrap();
   

    let value = [123; 40];

    for i in 0..250 {
        let mut keys = vec![];

        for j in 0..4_000 {
            let n = (i * 4_000 as u128) + (j as u128);
            keys.push(n.to_be_bytes());
        }

        let mut batch: Vec<TreeBatchEntry> = vec![];
        for key in keys.iter() {
            batch.push((&key[..], TreeOp::Put(&value)));
        }

        merk.apply_unchecked(&batch).unwrap();
    }

    let mut i = 0;
    b.iter(|| {
        let mut keys = vec![];
        for j in 0..4_000 {
            let n = (i % 250 as u128) + (j * 250 as u128);
            keys.push(n.to_be_bytes());
        }

        let mut batch: Vec<TreeBatchEntry> = vec![];
        for key in keys.iter() {
            batch.push((&key[..], TreeOp::Put(&value)));
        }

        merk.apply_unchecked(&batch).unwrap();

        i += 1;
    });

    println!("height: {}", merk.tree.as_ref().unwrap().height());
}

#[bench]
fn bench_delete_random(b: &mut test::Bencher) {
    //let mut merk = Merk::open("./test_merk_bench_delete_random.db").unwrap();
    let mut db = TemporaryDB::new();
    let mut merk = Merk::new(&mut db).unwrap();

    let value = [123; 40];

    for i in 0..400 {
        let mut keys = vec![];

        for j in 0..4_000 {
            let n = (i * 4_000 as u128) + (j as u128);
            keys.push(n.to_be_bytes());
        }

        let mut batch: Vec<TreeBatchEntry> = vec![];
        for key in keys.iter() {
            batch.push((&key[..], TreeOp::Put(&value)));
        }

        merk.apply_unchecked(&batch).unwrap();
    }

    let mut i = 0;
    b.iter(|| {
        let mut keys = vec![];
        for j in 0..4_000 {
            let n = (i % 400 as u128) + (j * 400 as u128);
            keys.push(n.to_be_bytes());
        }

        let mut batch: Vec<TreeBatchEntry> = vec![];
        for key in keys.iter() {
            batch.push((&key[..], TreeOp::Delete));
        }

        merk.apply_unchecked(&batch).unwrap();

        i += 1;
    });

    println!("height: {}", merk.tree.as_ref().unwrap().height());
}

#[bench]
fn bench_get_random(b: &mut test::Bencher) {
    //let mut merk = Merk::open("./test_merk_bench_get_random.db").unwrap();
    let mut db = TemporaryDB::new();
    let mut merk = Merk::new(&mut db).unwrap();
    let mut rng = rand::thread_rng();

    let value = [123; 40];

    for i in 0..250 {
        let mut keys = vec![];

        for j in 0..4_000 {
            let n = (i * 4_000 as u128) + (j as u128);
            keys.push(n.to_be_bytes());
        }

        let mut batch: Vec<TreeBatchEntry> = vec![];
        for key in keys.iter() {
            batch.push((&key[..], TreeOp::Put(&value)));
        }

        merk.apply_unchecked(&batch).unwrap();
    }

    b.iter(|| {
        let n = rng.gen::<u128>() % (250 * 4_000);
        let key = n.to_be_bytes();
        let retrieved_value = merk.get(&key).unwrap();
        assert_eq!(&retrieved_value[..], &value[..]);
    });
}

#[bench]
fn bench_put_insert_sequential(b: &mut test::Bencher) {
    //let mut merk = Merk::open("./test_merk_bench_put_insert_sequential.db").unwrap();
    let mut db = TemporaryDB::new();
    let mut merk = Merk::new(&mut db).unwrap();

    let value = [123; 40];

    for i in 0..250 {
        let mut keys = vec![];

        for j in 0..4_000 {
            let n = (i * 4_000 as u128) + (j as u128);
            keys.push(n.to_be_bytes());
        }

        let mut batch: Vec<TreeBatchEntry> = vec![];
        for key in keys.iter() {
            batch.push((&key[..], TreeOp::Put(&value)));
        }

        merk.apply_unchecked(&batch).unwrap();
    }

    let mut i = 0;
    b.iter(|| {
        let mut keys = vec![];
        for j in 0..4_000 {
            let n = (i * 4_000) as u128 + j as u128;
            keys.push(n.to_be_bytes());
        }

        let value = [123 as u8; 40];

        let mut batch: Vec<TreeBatchEntry> = vec![];
        for key in keys.iter() {
            batch.push((&key[..], TreeOp::Put(&value)));
        }

        merk.apply_unchecked(&batch).unwrap();

        i += 1;
    });

    println!("final tree size: {}", i * 4_000);
}

#[bench]
fn bench_put_update_sequential(b: &mut test::Bencher) {
    //let mut merk = Merk::open("./test_merk_bench_put_update_sequential.db").unwrap();
    let mut db = TemporaryDB::new();
    let mut merk = Merk::new(&mut db).unwrap();

    let value = [123; 40];

    for i in 0..250 {
        let mut keys = vec![];

        for j in 0..4_000 {
            let n = (i * 4_000 as u128) + (j as u128);
            keys.push(n.to_be_bytes());
        }

        let mut batch: Vec<TreeBatchEntry> = vec![];
        for key in keys.iter() {
            batch.push((&key[..], TreeOp::Put(&value)));
        }

        merk.apply_unchecked(&batch).unwrap();
    }

    let mut i = 0;
    b.iter(|| {
        let mut keys = vec![];
        for j in 0..4_000 {
            let n = ((i % 250 as u128) * 4_000) + j as u128;
            keys.push(n.to_be_bytes());
        }

        let mut batch: Vec<TreeBatchEntry> = vec![];
        for key in keys.iter() {
            batch.push((&key[..], TreeOp::Put(&value)));
        }

        merk.apply_unchecked(&batch).unwrap();

        i += 1;
    });

    println!("height: {}", merk.tree.as_ref().unwrap().height());
}

#[bench]
fn bench_get_sequential(b: &mut test::Bencher) {
    //let mut merk = Merk::open("./test_merk_bench_get_sequential.db").unwrap();
    let mut db = TemporaryDB::new();
    let mut merk = Merk::new(&mut db).unwrap();

    let value = [123; 40];

    for i in 0..250 {
        let mut keys = vec![];

        for j in 0..4_000 {
            let n = (i * 4_000 as u128) + (j as u128);
            keys.push(n.to_be_bytes());
        }

        let mut batch: Vec<TreeBatchEntry> = vec![];
        for key in keys.iter() {
            batch.push((&key[..], TreeOp::Put(&value)));
        }

        merk.apply_unchecked(&batch).unwrap();
    }

    let mut i = 0;
    b.iter(|| {
        let key = (i % (250 * 4_000) as u128).to_be_bytes();
        let retrieved_value = merk.get(&key).unwrap();
        assert_eq!(&retrieved_value[..], &value[..]);
        i += 1;
    });
}

#[bench]
fn bench_delete_sequential(b: &mut test::Bencher) {
    //let mut merk = Merk::open("./test_merk_bench_delete_sequential.db").unwrap();
    let mut db = TemporaryDB::new();
    let mut merk = Merk::new(&mut db).unwrap();

    let value = [123; 1];

    for i in 0..400 {
        let mut keys = vec![];

        for j in 0..4000 {
            let n = (i * 4000 as u128) + (j as u128);
            keys.push(n.to_be_bytes());
        }

        let mut batch: Vec<TreeBatchEntry> = vec![];
        for key in keys.iter() {
            batch.push((&key[..], TreeOp::Put(&value)));
        }

        merk.apply_unchecked(&batch).unwrap();
    }

    let mut i = 0;
    b.iter(|| {
        let mut keys = vec![];
        for j in 0..4000 {
            let n = ((i % 400 as u128) * 4000) + j as u128;
            keys.push(n.to_be_bytes());
        }

        let mut batch: Vec<TreeBatchEntry> = vec![];
        for key in keys.iter() {
            batch.push((&key[..], TreeOp::Delete));
        }

        merk.apply_unchecked(&batch).unwrap();

        i += 1;
    });

    println!("height: {}", merk.tree.as_ref().unwrap().height());
}
