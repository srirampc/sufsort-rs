extern crate libc;
use libc::{c_int, c_uchar};

extern{
    // Suffix Array constructed using 32-bit integers
    pub fn divsufsort(text: *const c_uchar, sa: *mut c_int,
                      length: c_int) -> c_int;
    pub fn divbwt(text: *const c_uchar, bwt: *mut c_uchar,
                  stemp: *mut c_int, length: c_int) -> c_int;
    pub fn bw_transform(text: *const c_uchar, bwt: *mut c_uchar,
                        sa: *mut c_int, length: c_int,
                        idx: *mut c_int) -> c_int;
    pub fn inverse_bw_transform(text: *const c_uchar, bwt: *mut c_uchar,
                                stemp: *mut c_int,
                                length: c_int, idx:c_int) -> c_int;
    pub fn sufcheck(text: *const c_uchar, sa: *const c_int,
                    length: c_int, verbose: c_int) -> c_int;
    pub fn sa_search(text: *const c_uchar, tlen: c_int,
                     pat: *const c_uchar, plen: c_int,
                     sa: *const c_int, salen: c_int,
                     idx: *mut c_int) -> c_int;
    pub fn sa_simplesearch(text: *const c_uchar, tlen: c_int,
                           sa: *const c_int, salen: c_int,
                           ch: c_int, idx:  *mut c_int) -> c_int;
    // TODO: Suffix Array constructed using 64-bit integers
}

pub fn construct_isa(sa: &Vec<i32>) -> Vec<i32>{
    let mut isa: Vec<i32> = vec![0; sa.len()];
    let mut i: i32 = 0;

    for t in sa{
        isa[(*t) as usize] = i;
        i = i + 1
    }
    isa
}

pub fn construct_sa(src: &[u8]) -> Vec<i32>{
    unsafe {
        let mut dst = Vec::with_capacity(src.len() as usize);
        divsufsort(src.as_ptr(), dst.as_mut_ptr(), src.len() as i32);
        dst.set_len(src.len());
        dst
    }
}

pub fn construct_sa_string(src: &String) -> Vec<i32>{
    construct_sa(src.as_bytes())
}

pub fn construct_bwt(src: &[u8]) -> String{
    let mut dst: Vec<u8> = Vec::with_capacity(src.len());
    unsafe{
        let mut tmp: Vec<i32> = Vec::with_capacity(src.len() + 1);
        let rv: i32 = divbwt(src.as_ptr(), dst.as_mut_ptr(),
                             tmp.as_mut_ptr(), src.len() as i32);

        tmp.set_len(src.len() + 1);
        dst.set_len(src.len());
        if rv > 0 {
            for i in 0..(rv as usize) {
                dst[i] = dst[i + 1];
            }
            dst[(rv - 1) as usize] = '$' as u8;
        }
        tmp.clear();
    }
    String::from_utf8(dst).unwrap()
}

pub fn construct_bwt_string(src: &String) -> String{
    construct_bwt(src.as_bytes())
}

pub fn construct_bwt_sa(src: &[u8], sa: &mut Vec<i32>) -> String{
    let mut dst: Vec<u8> = Vec::with_capacity(src.len());
    unsafe{
        let mut idx:i32 = -1;
        let pidx: *mut i32 = &mut idx;
        let rv = bw_transform(src.as_ptr(), dst.as_mut_ptr(),
                              sa.as_mut_ptr(),
                              src.len() as i32, pidx);
        dst.set_len(src.len());
        if rv == 0 && idx > 0 {
            for i in 0..(idx as usize){
                dst[i] = dst[i + 1];
            }
            dst[(idx - 1) as usize] = '$' as u8;
        }
    }
    String::from_utf8(dst).unwrap()
}

pub fn construct_bwt_sa_string(src: &String, sa: &mut Vec<i32>) -> String{
    construct_bwt_sa(src.as_bytes(), sa)
}

pub fn inverse_bwt(){
}

pub fn check_sa(src: &[u8], sa: &Vec<i32>, verbose: bool) -> bool{
    unsafe{
        let rv = sufcheck(src.as_ptr(), sa.as_ptr(),
                          src.len() as i32,
                          if verbose { 1 } else { 0 });
        rv == 0
    }
}

pub fn check_sa_string(src: &String, sa: &Vec<i32>, verbose: bool) -> bool{
    check_sa(src.as_bytes(), sa, verbose)
}

pub fn search_sa(src: &[u8], sa: &Vec<i32>, pat: &[u8]) -> (i32, i32){
    unsafe{
        let mut left:i32 = -1;
        let pleft: *mut i32 = &mut left;
        let count:i32 = sa_search(src.as_ptr(), src.len() as i32,
                                  pat.as_ptr(), pat.len() as i32,
                                  sa.as_ptr(), sa.len() as i32, pleft);
        (left, count)
    }
}

pub fn search_sa_string(src: &String, sa: &Vec<i32>,
                        pat: &String) -> (i32, i32){
    search_sa(src.as_bytes(), sa, pat.as_bytes())
}


pub fn simple_search_sa(src: &[u8], sa: &Vec<i32>, pc: char) -> (i32, i32){
    unsafe{
        let mut left:i32 = -1;
        let pleft: *mut i32 = &mut left;
        let count:i32 = sa_simplesearch(src.as_ptr(), src.len() as i32,
                                        sa.as_ptr(), sa.len() as i32,
                                        pc as i32, pleft);
        (left, count)
    }
}

pub fn simple_search_sa_string(src: &String, sa: &Vec<i32>,
                               pc: char) -> (i32, i32){
    simple_search_sa(src.as_bytes(), sa, pc)
}
