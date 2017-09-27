use std::cmp::min;

fn main() {
    let bv = [0, 1, 0, 1];
    println!("rank:{:?}", rank_mock(&bv, 0));
    println!("select:{:?}", select_mock(&bv, 1));
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
    return 0;
}


#[test]
fn rank_test() {
    let bv = [0,1,0,1];
    assert_eq!(rank_mock(&bv, 0), 0);
    assert_eq!(rank_mock(&bv, 1), 0);
    assert_eq!(rank_mock(&bv, 2), 1);
    assert_eq!(rank_mock(&bv, 3), 1);
    assert_eq!(rank_mock(&bv, 4), 2);
    assert_eq!(rank_mock(&bv, 5), 2);
}

#[test]
fn select_test() {
    let bv = [0,1,0,1];
    assert_eq!(select_mock(&bv, 0), 1);
    assert_eq!(select_mock(&bv, 1), 3);
    assert_eq!(select_mock(&bv, 2), 0);
}
