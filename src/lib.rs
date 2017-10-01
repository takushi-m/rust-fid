use std::cmp::min;
use std::vec::Vec;

pub struct Fid<'a> {
    raw: &'a [u8]
    ,big_block_size: usize
    ,big_blocks: Vec<usize>
    ,small_block_size: usize
    ,small_blocks: Vec<usize>
}

impl<'a> Fid<'a> {
    pub fn new(bv: &[u8]) -> Fid{
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

    pub fn build(&mut self) {
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

    pub fn rank(&self, idx: usize) -> usize {
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

    pub fn select(&self, n: usize) -> usize {
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
