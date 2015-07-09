#[cfg(test)]
mod tests {
    extern crate sufsort_rs as ss;
    use self::ss::SA;
    use self::ss::BWT;
    use self::ss::SearchSA;
    #[test]
    fn test_sufsort(){
        let s = ("MISSISSIPPI").to_string();
        let mut sax = s.construct_sa();
        assert_eq!(sax, &[10, 7, 4, 1, 0, 9, 8, 6, 3, 5, 2]);
        let bwt = s.construct_bwt();
        assert_eq!(bwt, "PSSMIPISSII");
        let bwt2 = s.construct_bwt_sa(&mut sax);
        assert_eq!(bwt2, "PSSMIPISSII");
        let pat = ("ISS").to_string();
        let rst = s.search_sa(&sax, &pat);
        assert_eq!(rst, (2, 2));
    }
}
