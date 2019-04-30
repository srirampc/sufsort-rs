extern crate libc;
extern crate num;

pub mod sufsort {
use libc::{c_uchar, int32_t, int64_t};

// Interface to raw functions from libdivsufsort
extern{
    // Suffix Array constructed using 32-bit integers
    pub fn divsufsort(text: *const c_uchar, sa: *mut int32_t,
                      length: int32_t) -> int32_t;
    pub fn divbwt(text: *const c_uchar, bwt: *mut c_uchar,
                  stemp: *mut int32_t, length: int32_t) -> int32_t;
    pub fn bw_transform(text: *const c_uchar, bwt: *mut c_uchar,
                        sa: *mut int32_t, length: int32_t,
                        idx: *mut int32_t) -> int32_t;
    pub fn inverse_bw_transform(text: *const c_uchar, bwt: *mut c_uchar,
                                stemp: *mut int32_t,
                                length: int32_t, idx:int32_t) -> int32_t;
    pub fn sufcheck(text: *const c_uchar, sa: *const int32_t,
                    length: int32_t, verbose: int32_t) -> int32_t;
    pub fn sa_search(text: *const c_uchar, tlen: int32_t,
                     pat: *const c_uchar, plen: int32_t,
                     sa: *const int32_t, salen: int32_t,
                     idx: *mut int32_t) -> int32_t;
    pub fn sa_simplesearch(text: *const c_uchar, tlen: int32_t,
                           sa: *const int32_t, salen: int32_t,
                           ch: int32_t, idx:  *mut int32_t) -> int32_t;

    // Suffix Array constructed using 64-bit integers
    pub fn divsufsort64(text: *const c_uchar, sa: *mut int64_t,
                        length: int64_t) -> int32_t;
    pub fn divbwt64(text: *const c_uchar, bwt: *mut c_uchar,
                    stemp: *mut int64_t, length: int64_t) -> int64_t;
    pub fn bw_transform64(text: *const c_uchar, bwt: *mut c_uchar,
                          sa: *mut int64_t, length: int64_t,
                          idx: *mut int64_t) -> int32_t;
    pub fn inverse_bw_transform64(text: *const c_uchar, bwt: *mut c_uchar,
                                stemp: *mut int64_t,
                                length: int64_t, idx:int64_t) -> int32_t;
    pub fn sufcheck64(text: *const c_uchar, sa: *const int64_t,
                      length: int64_t, verbose: int32_t) -> int32_t;
    pub fn sa_search64(text: *const c_uchar, tlen: int64_t,
                       pat: *const c_uchar, plen: int64_t,
                       sa: *const int64_t, salen: int64_t,
                       idx: *mut int64_t) -> int64_t;
    pub fn sa_simplesearch64(text: *const c_uchar, tlen: int64_t,
                             sa: *const int64_t, salen: int64_t,
                             ch: int64_t, idx:  *mut int64_t) -> int64_t;


}

    pub struct SA<T>{
        pub sa: Vec<T>,
    }

    impl From<String> for SA<i32> {
        /// Constructs Suffix Array for the given slice of u8 chars, src
        ///
        /// #Example
        ///
        /// ```
        /// let s = "MISSISSIPPI".to_string();
        /// let sax = SA::<i32>::from(s);
        /// assert_eq!(sax.sa, &[10, 7, 4, 1, 0, 9, 8, 6, 3, 5, 2]);
        /// ```
        fn from(instr: String) -> Self{
            unsafe {
                let src = instr.as_bytes();
                let mut dst : Vec<i32> = Vec::with_capacity(src.len() as usize);
                divsufsort(src.as_ptr(), dst.as_mut_ptr(), src.len() as i32);
                dst.set_len(src.len());
                SA::<i32>{ sa: dst }
            }
        }
    }

    impl From<String> for SA<i64> {
        /// Constructs Suffix Array for the given slice of u8 chars, src
        ///
        /// #Example
        ///
        /// ```
        /// let s = "MISSISSIPPI".to_string();
        /// let sax = SA::<i64>::from(s);
        /// assert_eq!(sax.sa, &[10, 7, 4, 1, 0, 9, 8, 6, 3, 5, 2]);
        /// ```
        fn from(instr: String) -> Self{
            unsafe {
                let src = instr.as_bytes();
                let mut dst : Vec<i64> = Vec::with_capacity(src.len() as usize);
                divsufsort64(src.as_ptr(), dst.as_mut_ptr(), src.len() as i64);
                dst.set_len(src.len());
                SA::<i64>{ sa: dst }
            }
        }
    }

pub trait SuffixArray<T>{
    fn construct_sa(&self) -> Vec<T>;
    fn check_sa(&self, sa: &Vec<T>, verbose: bool) -> bool;
}

impl SuffixArray<i32> for String{
    /// Constructs Suffix Array for the given slice of u8 chars, src
    ///
    /// #Example
    ///
    /// ```
    /// let s = "MISSISSIPPI".to_string();
    /// let sax : Vec<i32> = s.construct_sa();
    /// assert_eq!(sax, &[10, 7, 4, 1, 0, 9, 8, 6, 3, 5, 2]);
    /// ```
    fn construct_sa(&self) -> Vec<i32> {
        unsafe {
            let src = self.as_bytes();
            let mut dst = Vec::with_capacity(src.len() as usize);
            divsufsort(src.as_ptr(), dst.as_mut_ptr(), src.len() as i32);
            dst.set_len(src.len());
            dst
        }
    }

    /// Check if given sa is the suffix array for the source string src
    ///
    /// #Example
    ///
    /// ```
    /// let src = "MISSISSIPPI".to_string();
    /// let sat: Vec<i32> = vec![10, 7, 4, 1, 0, 9, 8, 6, 3, 5, 2];
    /// assert_eq!(src.check_sa(&sat, false), true)
    /// ```
    fn check_sa(&self, sa: &Vec<i32>, verbose: bool) -> bool{
        if self.len() == sa.len() {
            unsafe{
                let src = self.as_bytes();
                let rv = sufcheck(src.as_ptr(), sa.as_ptr(),
                                src.len() as i32,
                                if verbose { 1 } else { 0 });
                rv == 0
            }
        } else {
            false
        }
    }

}

impl SuffixArray<i64> for String{
    fn construct_sa(&self) -> Vec<i64> {
        unsafe {
            let src = self.as_bytes();
            let mut dst : Vec<i64> = Vec::with_capacity(src.len() as usize);
            divsufsort64(src.as_ptr(), dst.as_mut_ptr(), src.len() as i64);
            dst.set_len(src.len());
            dst
        }
    }

    fn check_sa(&self, sa: &Vec<i64>, verbose: bool) -> bool{
        if self.len() == sa.len() {
            unsafe{
                let src = self.as_bytes();
                let rv = sufcheck64(src.as_ptr(), sa.as_ptr(),
                                    src.len() as i64,
                                    if verbose { 1 } else { 0 });
                rv == 0
            }
        } else {
            false
        }
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
pub fn construct_isa<T>(sa: &Vec<T>) -> Vec<T>
    where T: std::clone::Clone + std::marker::Copy + std::ops::Add +
             num::ToPrimitive + num::One + num::Zero {
    let mut isa: Vec<T> = vec![T::zero(); sa.len()];
    let mut i: T = T::zero();

    for t in sa{
        isa[(*t).to_usize().unwrap()] = i;
        i = i + T::one();
    }
    isa
}

pub trait BWT<T>{
    fn construct_bwt(&self) -> (Vec<T>, String);
}

impl BWT<i32> for String{
    /// Construct bwt of the string src. Assumes that the BWT wraps around, i.e,
    /// for the position i such that SA[i] is 0, BWT[i] is src[src.len() - 1].
    /// Uses a temporary array of length |src| + 1.
    ///
    /// #Example
    ///
    /// 
    /// let src: Vec<u8> = vec![77, 73, 83, 83, 73, 83, 83, 73, 80, 80, 73];
    /// let btx = sufsort_rs::construct_bwt(&src);
    /// assert_eq!(btx, &[80, 83, 83, 77, 73, 80, 73, 83, 83, 73, 73]);
    /// 
    fn construct_bwt(&self) -> (Vec<i32>, String) {
        let mut dst: Vec<u8> = Vec::with_capacity(self.len());
        unsafe{
            let src = self.as_bytes();
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
            (tmp, String::from_utf8(dst).unwrap())
        }
    }
}


impl BWT<i64> for String{
    /// Construct bwt of the string src. Assumes that the BWT wraps around, i.e,
    /// for the position i such that SA[i] is 0, BWT[i] is src[src.len() - 1].
    /// Uses a temporary array of length |src| + 1.
    ///
    /// #Example
    ///
    /// 
    /// let src: Vec<u8> = vec![77, 73, 83, 83, 73, 83, 83, 73, 80, 80, 73];
    /// let btx = sufsort_rs::construct_bwt(&src);
    /// assert_eq!(btx, &[80, 83, 83, 77, 73, 80, 73, 83, 83, 73, 73]);
    /// 
    fn construct_bwt(&self) -> (Vec<i64>, String) {
        let mut dst: Vec<u8> = Vec::with_capacity(self.len());
        unsafe{
            let src = self.as_bytes();
            let mut tmp: Vec<i64> = Vec::with_capacity(src.len() + 1);
            let rv: i64 = divbwt64(src.as_ptr(), dst.as_mut_ptr(),
                                  tmp.as_mut_ptr(), src.len() as i64);

            tmp.set_len(src.len() + 1);
            dst.set_len(src.len());
            if rv > 0 {
                for i in 0..(rv as usize) {
                    dst[i] = dst[i + 1];
                }
                dst[(rv - 1) as usize] = src[src.len() - 1];
            }
            (tmp, String::from_utf8(dst).unwrap())
        }
    }
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


pub fn inverse_bwt(){
    // TODO:: Need further design
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
}
