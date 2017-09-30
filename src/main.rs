extern crate rand;

use std::cmp::min;
use std::vec::Vec;
use rand::Rng;

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

struct Fid<'a> {
    raw: &'a [u8]
    ,big_block_size: usize
    ,big_blocks: Vec<usize>
    ,small_block_size: usize
    ,small_blocks: Vec<usize>
}

impl<'a> Fid<'a> {
    fn new(bv: &[u8]) -> Fid{
        let mut ret = Fid {
            raw: bv
            ,big_block_size: 0
            ,big_blocks: Vec::new()
            ,small_block_size: 0
            ,small_blocks: Vec::new()
        };
        ret.build();
        ret
    }

    fn build(&mut self) {
        let n = self.raw.len();
        let big_block_size = (n as f64).log2().powi(2).floor() as usize; // (lg(n))^2
        let small_block_size = ((n as f64).log2()/2.0).floor() as usize; // lg(n)/2
        self.big_block_size = big_block_size;
        self.small_block_size = small_block_size;

        let mut cntb = 0;
        let mut cnts = 0;
        for i in 0..n {
            if i%big_block_size==0 {
                self.big_blocks.push(cntb);
            }
            if i%small_block_size==0 {
                let s = cnts - self.big_blocks.last().unwrap();
                self.small_blocks.push(s);
            }
            if self.raw[i]==1 {
                cntb = cntb + 1;
                cnts = cnts + 1;
            }
        }
    }

    fn rank(&self, idx: usize) -> usize {
        let mut rank = 0;

        let p = idx % self.big_block_size;
        let pos = min((idx-p)/self.big_block_size, self.big_blocks.len()-1);
        rank += self.big_blocks[pos];

        let sp = p % self.small_block_size;
        let spos = pos*self.big_block_size/self.small_block_size + (p-sp)/self.small_block_size;
        rank += self.small_blocks[spos];

        for i in (spos*self.small_block_size)..min(idx,self.raw.len()) {
            if self.raw[i] == 1 {
                rank += 1;
            }
        }
        rank
    }

    fn select(&self, n: usize) -> usize {
        let mut l = 0;
        let mut u = self.raw.len();

        while u-l>1 {
            let m = (((l+u) as f32)/2.0).floor() as usize;
            if n < self.rank(m) {
                u = m;
            } else {
                l = m;
            }
        }
        if n==self.rank(l)&&self.raw[l]==1 {
            l
        } else {
            self.raw.len()
        }
    }
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
