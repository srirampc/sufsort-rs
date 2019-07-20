
pub fn construct_lcp_kasai<T>(text: &[u8], sa: &Vec<T>,
                                isa: &Vec<T>) -> Vec<T>
    where T: std::clone::Clone + std::marker::Copy +
                std::ops::Add + std::ops::Sub<Output=T> +
                std::cmp::PartialEq +
                num::ToPrimitive + num::One + num::Zero {
    let n = sa.len();
    assert!(n == text.len());
    assert!(n == isa.len());
    let mut lcp : Vec<T> = Vec::with_capacity(n);
    for i in 0..n {
        lcp.push(sa[i].clone());
    }
    let mut l : T = T::zero();
    for i in 0..n {
        let sa_1 = match isa[i].to_usize() {
            Some(x) => x,
            None => 0,
        };

        if sa_1 != 0 {
            let mut jdx = sa_1 - 1;
            let j : usize = match lcp[jdx].to_usize() {
                    Some(x) => x,
                    None => break,
                };
            if l != T::zero() {
                l = l - T::one();
            }
            let mut ldx = match l.to_usize() {
                    Some(x) => x,
                    None => break,
                };;
            while text[i+ldx] == text[j+ldx] {
                l = l + T::one();
                ldx = match l.to_usize()  {
                    Some(x) => x,
                    None => break,
                };
            }
            jdx = sa_1 - 1;
            lcp[jdx] = l;
        } else {
            l = T::zero();
            lcp[n-1] = T::zero();
        }
    }

    for i in (n-1)..2 {
        lcp[i-1] = lcp[i-2];
    }

    lcp[0] = T::zero();
    lcp
}

pub fn construct_lcp_phi<T>(text: &[u8], sa: &Vec<T>) -> Vec<T>
    where T: std::clone::Clone + std::marker::Copy +
                std::ops::Add + std::ops::Sub<Output=T> +
                std::cmp::PartialEq +
                num::ToPrimitive + num::One + num::Zero {
    let n = sa.len();
    assert!(n == text.len());
    let mut lcp : Vec<T> = vec![T::zero(); n];
    let mut plcp : Vec<T> = vec![T::zero(); n];
    let mut sai_1 : T = T::zero();
    // (1) Calculate PHI
    for i in 0..n {
        let sai = match sa[i].to_usize(){
            Some(x) => x,
            None => continue,
        };
        plcp[sai] = sai_1;
        sai_1 = sa[i].clone();
    }

    // (2) Calculate Permuted LCP array.
    let mut max_size : usize = 0;
    let mut l : T = T::zero();
    for i in 0..n-1 {
        let phii = match plcp[i].to_usize() {
            Some(x) => x,
            None => continue,
        };
        let mut ldx = match l.to_usize(){
            Some(x) => x,
            None => continue,
        };
        while text[i+ldx] == text[phii+ldx]{
            l = l + T::one();
            ldx = match l.to_usize(){
                Some(x) => x,
                None => continue,
            };
        }
        plcp[i] = T::zero() + l;
        if l != T::zero() {
            max_size = std::cmp::max(max_size, l.to_usize().unwrap());
            l = l - T::one();
        }
    }
    for i in 0..n {
        let sai = match sa[i].to_usize(){
            Some(x) => x,
            None => continue,
        };
        lcp[i] = plcp[sai];
    }
    lcp
}

pub fn construct_lcp_from_sa<T>(text: &[u8], sa: &Vec<T>, isa: &Vec<T>) -> Vec<T>
    where T: std::clone::Clone + std::marker::Copy +
             num::FromPrimitive + num::ToPrimitive + num::Zero {
    let n = text.len();
    assert!(n == sa.len());
    assert!(n == isa.len());
    let mut lcp : Vec<T> = vec![T::zero(); n];
    // first LCP is undefined -> set to 0:
    // LCP[0] = 0;
    lcp[0] = T::zero();
    // std::size_t h = 0;
    let mut h: usize = 0;

    // // in string order!
    // for (std::size_t i = 0; i < S.size(); ++i) {
    for i in 0..n-1 {
        // length of currently equal characters (in string order, next LCP value
        // is always >= current lcp - 1)
        // std::size_t k = 0;
        let mut k: usize = 0;

        // if (h > 0)
        //     k = h-1;
        if h > 0 {
            k = h - 1;
        }

        // comparing suffix starting from i=SA[ISA[i]] with the previous
        // suffix in SA order: SA[ISA[i]-1]
        let isadx = isa[i].to_usize().unwrap();
        // while (i+k < S.size() && ISA[i] > 0 &&
        //        SA[ISA[i]-1]+k < S.size() && S[i+k] == S[SA[ISA[i]-1]+k])
        //         k++;
        if isadx > 0 {
            let sadx =  sa[isadx - 1].to_usize().unwrap();
            while (i+k < text.len()) && 
                    (sadx+k < text.len()) &&
                    (text[i+k] == text[sadx+k]) {
                k = k + 1;
            }
        }
        // LCP[ISA[i]] = k;
        lcp[isadx] = match T::from_usize(k){
            Some(x) => x,
            None => continue,
        };
        // h = k;
        h = k;
    }
    // }
    lcp
}

