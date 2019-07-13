extern crate libc;
extern crate num;



use libc::c_uchar;

// Interface to raw functions from libdivsufsort
extern{
    // Suffix Array constructed using 32-bit integers
    pub fn divsufsort(text: *const c_uchar, sa: *mut i32,
                    length: i32) -> i32;
    pub fn divbwt(text: *const c_uchar, bwt: *mut c_uchar,
                stemp: *mut i32, length: i32) -> i32;
    pub fn bw_transform(text: *const c_uchar, bwt: *mut c_uchar,
                        sa: *mut i32, length: i32,
                        idx: *mut i32) -> i32;
    pub fn inverse_bw_transform(text: *const c_uchar, bwt: *mut c_uchar,
                                stemp: *mut i32,
                                length: i32, idx:i32) -> i32;
    pub fn sufcheck(text: *const c_uchar, sa: *const i32,
                    length: i32, verbose: i32) -> i32;
    pub fn sa_search(text: *const c_uchar, tlen: i32,
                    pat: *const c_uchar, plen: i32,
                    sa: *const i32, salen: i32,
                    idx: *mut i32) -> i32;
    pub fn sa_simplesearch(text: *const c_uchar, tlen: i32,
                        sa: *const i32, salen: i32,
                        ch: i32, idx:  *mut i32) -> i32;

    // Suffix Array constructed using 64-bit integers
    pub fn divsufsort64(text: *const c_uchar, sa: *mut i64,
                        length: i64) -> i32;
    pub fn divbwt64(text: *const c_uchar, bwt: *mut c_uchar,
                    stemp: *mut i64, length: i64) -> i64;
    pub fn bw_transform64(text: *const c_uchar, bwt: *mut c_uchar,
                        sa: *mut i64, length: i64,
                        idx: *mut i64) -> i32;
    pub fn inverse_bw_transform64(text: *const c_uchar, bwt: *mut c_uchar,
                                stemp: *mut i64,
                                length: i64, idx:i64) -> i32;
    pub fn sufcheck64(text: *const c_uchar, sa: *const i64,
                    length: i64, verbose: i32) -> i32;
    pub fn sa_search64(text: *const c_uchar, tlen: i64,
                    pat: *const c_uchar, plen: i64,
                    sa: *const i64, salen: i64,
                    idx: *mut i64) -> i64;
    pub fn sa_simplesearch64(text: *const c_uchar, tlen: i64,
                            sa: *const i64, salen: i64,
                            ch: i64, idx:  *mut i64) -> i64;

}

pub struct SA<'s, T>{
    pub txt:&'s [u8],
    pub sarray: Vec<T>,
}

impl<'s> SA<'s, i32> {
    /// Constructs Suffix Array for the given slice of u8 chars, src
    ///
    /// #Example
    ///
    /// ```
    /// let s = "MISSISSIPPI".to_string();
    /// let say = sufsort_rs::sufsort::SA::<i32>::new(&s);
    /// assert_eq!(say.sarray, &[10, 7, 4, 1, 0, 9, 8, 6, 3, 5, 2]);
    /// ```
    pub fn new(text: &'s String) -> Self {
        unsafe {
            let src = text.as_bytes();
            let mut dst : Vec<i32> = Vec::with_capacity(src.len() as usize);
            divsufsort(src.as_ptr(), dst.as_mut_ptr(), src.len() as i32);
            dst.set_len(src.len());
            SA::<'s, i32>{txt: src, sarray: dst}
        }
    }

    /// Check if given sa is the suffix array for the source string src
    ///
    /// #Example
    ///
    /// ```
    /// let s = "MISSISSIPPI".to_string();
    /// let say = sufsort_rs::sufsort::SA::<i32>::new(&s);
    /// let sat: Vec<i32> = vec![10, 7, 4, 1, 0, 9, 8, 6, 3, 5, 2];
    /// assert_eq!(say.check_sa(false), true)
    /// ```
    pub fn check_sa(&self, verbose: bool) -> bool{
        if self.txt.len() > 0 && self.txt.len() == self.sarray.len() {
            unsafe{
                let src = self.txt;
                let rv = sufcheck(src.as_ptr(), self.sarray.as_ptr(),
                                    src.len() as i32,
                                    if verbose { 1 } else { 0 });
                rv == 0
            }
        } else {
            false
        }
    }

    pub fn search_sa(&self, pat: &[u8]) -> (i32, i32){
        unsafe{
            let mut left:i32 = -1;
            let pleft: *mut i32 = &mut left;
            let src = self.txt;
            let count:i32 = sa_search(src.as_ptr(), src.len() as i32,
                                        pat.as_ptr(), pat.len() as i32,
                                        self.sarray.as_ptr(),
                                        self.sarray.len() as i32, pleft);
            (left, count)
        }
    }

    pub fn simple_search(&self, pc: char) -> (i32, i32){
        unsafe{
            let mut left:i32 = -1;
            let pleft: *mut i32 = &mut left;
            let src = self.txt;
            let count:i32 = sa_simplesearch(src.as_ptr(), src.len() as i32,
                                            self.sarray.as_ptr(),
                                            self.sarray.len() as i32,
                                            pc as i32, pleft);
            (left, count)
        }
    }
}

impl<'s> SA<'s, i64> {
    /// Constructs Suffix Array for the given slice of u8 chars, src
    ///
    /// #Example
    ///
    /// ```
    /// let s = "MISSISSIPPI".to_string();
    /// let say = sufsort_rs::sufsort::SA::<i64>::new(&s);
    /// assert_eq!(say.sarray, &[10, 7, 4, 1, 0, 9, 8, 6, 3, 5, 2]);
    /// ```
    pub fn new(text: &'s String) -> Self{
        unsafe {
            let src = text.as_bytes();
            let mut dst : Vec<i64> = Vec::with_capacity(src.len() as usize);
            divsufsort64(src.as_ptr(), dst.as_mut_ptr(), src.len() as i64);
            dst.set_len(src.len());
            SA::<i64>{txt: src, sarray: dst}
        }
    }

    pub fn check_sa(&self, verbose: bool) -> bool{
        if self.txt.len() > 0 && self.txt.len() == self.sarray.len() {
            unsafe{
                let src = self.txt;
                let rv = sufcheck64(src.as_ptr(), self.sarray.as_ptr(),
                                    src.len() as i64,
                                    if verbose { 1 } else { 0 });
                rv == 0
            }
        } else {
            false
        }
    }

    pub fn search_sa(&self, pat: &[u8]) -> (i64, i64){
        unsafe{
            let mut left:i64 = -1;
            let pleft: *mut i64 = &mut left;
            let src = self.txt;
            let count:i64 = sa_search64(src.as_ptr(), src.len() as i64,
                                        pat.as_ptr(), pat.len() as i64,
                                        self.sarray.as_ptr(),
                                        self.sarray.len() as i64, pleft);
            (left, count)
        }
    }

    pub fn simple_search(&self, pc: char) -> (i64, i64){
        unsafe{
            let mut left:i64 = -1;
            let pleft: *mut i64 = &mut left;
            let src = self.txt;
            let count: i64 = sa_simplesearch64(src.as_ptr(), src.len() as i64,
                                                self.sarray.as_ptr(),
                                                self.sarray.len() as i64,
                                                pc as i64, pleft);
            (left, count)
        }
    }
}


/// Construct the reverse lookup mapping corresponding to SA
///
/// #Example
/// ```
/// let sav: Vec<i32> = vec![10, 7, 4, 1, 0, 9, 8, 6, 3, 5, 2];
/// let isa = sufsort_rs::sufsort::construct_isa(&sav);
/// assert_eq!(isa,  &[4, 3, 10, 8, 2, 9, 7, 1, 6, 5, 0]);
/// ```
pub fn construct_isa<T>(sa: &Vec<T>) -> Vec<T>
    where T: std::clone::Clone + std::marker::Copy + std::ops::Add +
            num::ToPrimitive + num::One + num::Zero {
    let mut isa: Vec<T> = vec![T::zero(); sa.len()];
    let mut i: T = T::zero();

    for t in sa {
        match (*t).to_usize() {
            Some(x) => {
                // isa[(*t).to_usize().unwrap()] = i;
                isa[x] = i.clone();
                i = i + T::one();
            },
            None => ()
        }
    }
    isa
}

pub struct BWT<'s, T>{
    pub txt:&'s [u8],
    pub sarray: Vec<T>,
    pub bwt: Vec<u8>
}

impl<'s> BWT<'s, i32> {
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
        pub fn new(text: &'s String) -> Self {
        unsafe {
            let mut dst: Vec<u8> = Vec::with_capacity(text.len());
            let src = text.as_bytes();
            let mut sax: Vec<i32> = Vec::with_capacity(src.len() + 1);
            let rv: i32 = divbwt(src.as_ptr(), dst.as_mut_ptr(),
                                    sax.as_mut_ptr(), src.len() as i32);

            sax.set_len(src.len() + 1);
            dst.set_len(src.len());
            if rv > 0 && dst.len() >= (rv as usize) {
                for i in 0..(rv as usize) {
                    dst[i] = dst[i + 1];
                }
                match src.last() {
                    Some(x) => dst[(rv - 1) as usize] = x.clone(), //[src.len() - 1];
                    None => ()
                }
            }
            BWT::<'s, i32>{txt: src, sarray: sax, bwt: dst}
        }
    }
}


impl<'s> BWT<'s, i64> {
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
    pub fn new(text: &'s String) -> Self{
        let mut dst: Vec<u8> = Vec::with_capacity(text.len());
        unsafe{
            let src = text.as_bytes();
            let mut tmp: Vec<i64> = Vec::with_capacity(src.len() + 1);
            let rv: i64 = divbwt64(src.as_ptr(), dst.as_mut_ptr(),
                                    tmp.as_mut_ptr(), src.len() as i64);

            tmp.set_len(src.len() + 1);
            dst.set_len(src.len());
            if rv > 0 && dst.len() >= (rv as usize) {
                for i in 0..(rv as usize) {
                    dst[i] = dst[i + 1];
                }
                match src.last() {
                    Some(x) => dst[(rv - 1) as usize] = x.clone(), //[src.len() - 1];
                    None => ()
                }
            }
            BWT::<'s, i64>{txt: src, sarray: tmp, bwt: dst}
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
/// let btx = sufsort_rs::sufsort::construct_bwt_sa(&src, &mut sa);
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
