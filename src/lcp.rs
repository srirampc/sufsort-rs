
pub fn construct_lcp_kasai<T>(text: &[u8], sa: &Vec<T>,
                                isa: &Vec<T>) -> Vec<T>
    where T: std::clone::Clone + std::marker::Copy +
                std::ops::Add + std::ops::Sub<Output=T> +
                std::cmp::PartialEq +
                num::ToPrimitive + num::One + num::Zero {
    let n = sa.len();
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

// TODO: port the following from patflick/psac
// template <typename index_t>
// void lcp_from_sa(const std::string& S, const std::vector<index_t>& SA, const std::vector<index_t>& ISA, std::vector<index_t>& LCP) {
//     // TODO: cite the source for this linear O(n) algorithm!

//     // input sizes must be equal
//     assert(S.size() == SA.size());
//     assert(SA.size() == ISA.size());

//     // init LCP array if not yet of correct size
//     if (LCP.size() != S.size()) {
//         LCP.resize(S.size());
//     }

//     // first LCP is undefined -> set to 0:
//     LCP[0] = 0;

//     std::size_t h = 0;

//     // in string order!
//     for (std::size_t i = 0; i < S.size(); ++i) {
//         // length of currently equal characters (in string order, next LCP value
//         // is always >= current lcp - 1)
//         std::size_t k = 0;
//         if (h > 0)
//             k = h-1;
//         // comparing suffix starting from i=SA[ISA[i]] with the previous
//         // suffix in SA order: SA[ISA[i]-1]
//         while (i+k < S.size() && ISA[i] > 0 && SA[ISA[i]-1]+k < S.size() && S[i+k] == S[SA[ISA[i]-1]+k])
//             k++;
//         LCP[ISA[i]] = k;
//         h = k;
//     }
// }