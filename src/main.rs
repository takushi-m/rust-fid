extern crate rand;
extern crate rust_fid;

use std::cmp::min;
use rand::Rng;
use rust_fid::Fid;

fn main() {
    let mut rng = rand::thread_rng();
    let mut bv = [0; 65536];
    for i in 0..65536 {
        bv[i] = (rng.gen::<usize>()%2) as u8;
    }
    let fid = Fid::new(&bv);

    let s = std::time::Instant::now();
    for i in 0..10000 {
        fid.rank(rng.gen::<usize>()%65536);
    }
    let e = s.elapsed();
    let elapsed = e.as_secs()*1_000_000_000 + e.subsec_nanos() as u64;
    println!("FID:");
    println!("{:?}ms", elapsed/1_000_000);
    println!("{:?}us/query", elapsed/10000/1000);

    let s = std::time::Instant::now();
    for i in 0..10000 {
        rank_mock(&bv, rng.gen::<usize>()%65536);
    }
    let e = s.elapsed();
    let elapsed = e.as_secs()*1_000_000_000 + e.subsec_nanos() as u64;
    println!("normal:");
    println!("{:?}ms", elapsed/1_000_000);
    println!("{:?}us/query", elapsed/10000/1000);
}

fn rank_mock(bv: &[u8], idx: usize) -> usize {
    let mut ret = 0;
    let len = min(bv.len(), idx);

    for i in 0..len {
        if bv[i]==1 {
            ret = ret + 1;
        }
    }

    ret
}

fn select_mock(bv: &[u8], idx: usize) -> usize {
    let mut cnt = 0;
    for i in 0..bv.len() {
        if bv[i]==1 {
            cnt = cnt+1;
        }
        if cnt==idx+1 {
            return i;
        }
    }
    return bv.len()
}


#[test]
fn rank_test() {
    let bv = [0,1,0,1];
    let fid = Fid::new(&bv);

    assert_eq!(rank_mock(&bv, 0), 0);
    assert_eq!(rank_mock(&bv, 1), 0);
    assert_eq!(rank_mock(&bv, 2), 1);
    assert_eq!(rank_mock(&bv, 3), 1);
    assert_eq!(rank_mock(&bv, 4), 2);
    assert_eq!(rank_mock(&bv, 5), 2);

    assert_eq!(rank_mock(&bv, 0), fid.rank(0));
    assert_eq!(rank_mock(&bv, 1), fid.rank(1));
    assert_eq!(rank_mock(&bv, 2), fid.rank(2));
    assert_eq!(rank_mock(&bv, 3), fid.rank(3));
    assert_eq!(rank_mock(&bv, 4), fid.rank(4));
    assert_eq!(rank_mock(&bv, 5), fid.rank(5));
}

#[test]
fn rank_test2() {
    let mut rng = rand::thread_rng();
    let mut bv = [0;64];
    for i in 0..64 {
        bv[i] = (rng.gen::<usize>()%2) as u8;
    }

    let fid = Fid::new(&bv);
    for i in 0..64 {
        assert_eq!(rank_mock(&bv, i), fid.rank(i));
    }
}

#[test]
fn select_test() {
    let bv = [0,1,0,1];
    let fid = Fid::new(&bv);
    assert_eq!(select_mock(&bv, 0), 1);
    assert_eq!(select_mock(&bv, 1), 3);
    assert_eq!(select_mock(&bv, 2), 4);

    assert_eq!(select_mock(&bv, 0), fid.select(0));
    assert_eq!(select_mock(&bv, 1), fid.select(1));
    assert_eq!(select_mock(&bv, 2), fid.select(2));
}

#[test]
fn select_test2() {
    let mut rng = rand::thread_rng();
    let mut bv = [0;64];
    for i in 0..64 {
        bv[i] = (rng.gen::<usize>()%2) as u8;
    }

    let fid = Fid::new(&bv);
    for i in 0..64 {
        println!("{:?}", i);
        assert_eq!(select_mock(&bv, i), fid.select(i));
    }
}
