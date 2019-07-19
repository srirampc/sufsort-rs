
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
                std::ops::Add +
                std::cmp::PartialEq + std::cmp::PartialOrd +
                num::FromPrimitive + num::ToPrimitive +
                num::One + num::Zero {
    let n = text.len();
    assert!(n == sa.len());
    assert!(n == isa.len());
    let mut lcp : Vec<T> = vec![T::zero(); n];
    lcp[0] = T::zero();
    let mut h: usize = 0;
    for i in 0..n-1 {
        let mut k: usize = 0;
        if h > 0 {
            k = h - 1;
        }

        // comparing suffix starting from i=SA[ISA[i]] with the previous
        // suffix in SA order: SA[ISA[i]-1]
        let isdx = match isa[i].to_usize() {
            Some(x) => x,
            None => continue,
        };
        let sdx = match sa[isdx-1].to_usize() {
            Some(x) => x + k,
            None => continue,
        };
        while (i+k < text.len()) && (isa[i] > T::zero()) && 
              (sdx < text.len()) && (text[i] == text[sdx]) {
            k = k + 1;
        }
        let ldx = match isa[i].to_usize(){
            Some(x) => x,
            None => continue,
        };
        lcp[ldx] = match T::from_usize(k){
            Some(x) => x,
            None => continue,
        };
        h = k;
    }
    lcp
}

