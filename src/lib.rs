extern crate libc;
use libc::{c_int, c_uchar};


// Interface to raw functions from libdivsufsort
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


/// Constructs Suffix Array for the given slice of u8 chars, src
///
/// #Example
///
/// ```
/// let s = ("MISSISSIPPI").to_string();
/// let sax = sufsort_rs::construct_sa(s.as_bytes());
/// assert_eq!(sax, &[10, 7, 4, 1, 0, 9, 8, 6, 3, 5, 2]);
/// ```
pub fn construct_sa(src: &[u8]) -> Vec<i32>{
    unsafe {
        let mut dst = Vec::with_capacity(src.len() as usize);
        divsufsort(src.as_ptr(), dst.as_mut_ptr(), src.len() as i32);
        dst.set_len(src.len());
        dst
    }
}


/// Constructs Suffix Array for the given source string src
///
/// #Example
///
/// ```
/// let s = ("MISSISSIPPI").to_string();
/// let sax = sufsort_rs::construct_sa_string(&s);
/// assert_eq!(sax, &[10, 7, 4, 1, 0, 9, 8, 6, 3, 5, 2]);
/// ```
#[inline]
pub fn construct_sa_string(src: &String) -> Vec<i32>{
    construct_sa(src.as_bytes())
}


/// Trait for construction of suffix array
pub trait SA{
    fn construct_sa(&self) -> Vec<i32>;
}


impl SA for String{
    fn construct_sa(&self) -> Vec<i32>{
        construct_sa(self.as_bytes())
    }
}


/// Construct the reverse lookup mapping corresponding to SA
///
/// #Example
/// ```
/// let sav: Vec<i32> = vec![10, 7, 4, 1, 0, 9, 8, 6, 3, 5, 2];
/// let isa = sufsort_rs::construct_isa(&sav);
/// assert_eq!(isa,  &[4, 3, 10, 8, 2, 9, 7, 1, 6, 5, 0]);
/// ```
pub fn construct_isa(sa: &Vec<i32>) -> Vec<i32>{
    let mut isa: Vec<i32> = vec![0; sa.len()];
    let mut i: i32 = 0;

    for t in sa{
        isa[(*t) as usize] = i;
        i = i + 1
    }
    isa
}


/// Construct bwt of the string src. Assumes that the BWT wraps around, i.e,
/// for the position i such that SA[i] is 0, BWT[i] is src[src.len() - 1].
/// Uses a temporary array of length |src| + 1.
///
/// #Example
///
/// ```
/// let src: Vec<u8> = vec![77, 73, 83, 83, 73, 83, 83, 73, 80, 80, 73];
/// let btx = sufsort_rs::construct_bwt(&src);
/// assert_eq!(btx, &[80, 83, 83, 77, 73, 80, 73, 83, 83, 73, 73]);
/// ```
pub fn construct_bwt(src: &[u8]) -> Vec<u8>{
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
            dst[(rv - 1) as usize] = src[src.len() - 1];
        }
        tmp.clear();
    }
    dst
}


/// Construct bwt of the string src. Assumes that the BWT wraps around, i.e,
/// for the position i such that SA[i] is 0, BWT[i] is src[src.len() - 1].
/// Uses a temporary array of length |src| + 1.
///
/// #Example
///
/// ```
/// let src = ("MISSISSIPPI").to_string();
/// let btx = sufsort_rs::construct_bwt_string(&src);
/// assert_eq!(btx, "PSSMIPISSII");
/// ```
pub fn construct_bwt_string(src: &String) -> String{
    String::from_utf8(construct_bwt(src.as_bytes())).unwrap()
}


/// Construct bwt transform of SA. Assumes that the BWT wraps around, i.e,
/// for the position i such that SA[i] is 0, BWT[i] is src[src.len() - 1].
///
/// #Example
///
/// ```
/// let src: Vec<u8> = vec![77, 73, 83, 83, 73, 83, 83, 73, 80, 80, 73];
/// let mut sa: Vec<i32> = vec![10, 7, 4, 1, 0, 9, 8, 6, 3, 5, 2];
/// let btx = sufsort_rs::construct_bwt_sa(&src, &mut sa);
/// assert_eq!(btx, &[80, 83, 83, 77, 73, 80, 73, 83, 83, 73, 73]);
/// ```
pub fn construct_bwt_sa(src: &[u8], sa: &mut Vec<i32>) -> Vec<u8>{
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
            dst[(idx - 1) as usize] = src[src.len() - 1];
        }
    }
    dst
}


/// Construct bwt transform of SA. Assumes that the BWT wraps around, i.e,
/// for the position i such that SA[i] is 0, BWT[i] is src[src.len() - 1].
///
/// #Example
///
/// ```
/// let src = ("MISSISSIPPI$").to_string();
/// let mut sa: Vec<i32> = vec![11, 10, 7, 4, 1, 0, 9, 8, 6, 3, 5, 2];
/// let btx = sufsort_rs::construct_bwt_sa_string(&src, &mut sa);
/// assert_eq!(btx, "IPSSM$PISSII");
/// ```
pub fn construct_bwt_sa_string(src: &String, sa: &mut Vec<i32>) -> String{
    String::from_utf8(construct_bwt_sa(src.as_bytes(), sa)).unwrap()
}


/// Trait for construction of bwt
pub trait BWT{
    fn construct_bwt(&self) -> String;
    fn construct_bwt_sa(&self, sa: &mut Vec<i32>) -> String;
}


impl BWT for String{
    fn construct_bwt(&self) -> String{
        String::from_utf8(construct_bwt(self.as_bytes())).unwrap()
    }

    fn construct_bwt_sa(&self, sa: &mut Vec<i32>) -> String{
        String::from_utf8(construct_bwt_sa(self.as_bytes(), sa)).unwrap()
    }
}


pub fn inverse_bwt(){
    // TODO:: Need further design
}


/// Check if given sa is the suffix array for the source string src
///
/// #Example
///
/// ```
/// let src: Vec<u8> = vec![77, 73, 83, 83, 73, 83, 83, 73, 80, 80, 73];
/// let sa: Vec<i32> = vec![10, 7, 4, 1, 0, 9, 8, 6, 3, 5, 2];
/// assert_eq!(sufsort_rs::check_sa(&src, &sa, false), true)
/// ```
pub fn check_sa(src: &[u8], sa: &Vec<i32>, verbose: bool) -> bool{
    unsafe{
        let rv = sufcheck(src.as_ptr(), sa.as_ptr(),
                          src.len() as i32,
                          if verbose { 1 } else { 0 });
        rv == 0
    }
}


/// Check if given sa is the suffix array for the source string src
///
/// #Example
///
/// ```
/// let src = ("MISSISSIPPI$").to_string();
/// let sa: Vec<i32> = vec![11, 10, 7, 4, 1, 0, 9, 8, 6, 3, 5, 2];
/// assert_eq!(sufsort_rs::check_sa_string(&src, &sa, false), true)
/// ```
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


/// Search for the pattern pat in the source string src, given sa
///
/// #Example
///
/// ```
/// let src = ("MISSISSIPPI").to_string();
/// let sa: Vec<i32> = vec![10, 7, 4, 1, 0, 9, 8, 6, 3, 5, 2];
/// let pat = ("IS").to_string();
/// let rst = sufsort_rs::search_sa_string(&src, &sa, &pat);
/// assert_eq!(rst, (2, 2))
/// ```
pub fn search_sa_string(src: &String, sa: &Vec<i32>,
                        pat: &String) -> (i32, i32){
    search_sa(src.as_bytes(), sa, pat.as_bytes())
}


/// Trait for construction of suffix array
pub trait SearchSA{
    fn search_sa(&self, sa: &Vec<i32>, pat: &String) -> (i32, i32);
}


impl SearchSA for String{
    fn search_sa(&self, sa: &Vec<i32>, pat: &String) -> (i32, i32){
        search_sa(self.as_bytes(), sa, pat.as_bytes())
    }
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


/// Search for the pattern pat in the source string src, given sa, suffix
///  array of src
///
/// #Example
///
/// ```
/// let src = ("MISSISSIPPI").to_string();
/// let sa: Vec<i32> = vec![10, 7, 4, 1, 0, 9, 8, 6, 3, 5, 2];
/// let rst = sufsort_rs::simple_search_sa_string(&src, &sa, 'P');
/// assert_eq!(rst, (5, 2))
/// ```
pub fn simple_search_sa_string(src: &String, sa: &Vec<i32>,
                               pc: char) -> (i32, i32){
    simple_search_sa(src.as_bytes(), sa, pc)
}
