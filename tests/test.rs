#[cfg(test)]
mod tests {
    extern crate sufsort_rs as ss;
    use self::ss::sufsort::SA;
    use self::ss::sufsort::BWT;
    use self::ss::sufsort::construct_bwt_sa;

    #[test]
    fn test_sufsort(){
        let s = ("MISSISSIPPI").to_string();
        {
          let say = SA::<i32>::new(&s);
          assert_eq!(say.sarray, &[10, 7, 4, 1, 0, 9, 8, 6, 3, 5, 2]);
        }
    }

    #[test]
    fn test_bwt(){
        let s = ("MISSISSIPPI").to_string();
        {
            let bwx = BWT::<i32>::new(&s);
            assert_eq!(bwx.bwt, "PSSMIPISSII".as_bytes());
        }
    }

    #[test]
    fn test_bwt_sa(){
        let s = ("MISSISSIPPI").to_string();
        let mut say = SA::<i32>::new(&s);
        let bwt2 = construct_bwt_sa(s.as_bytes(), &mut say.sarray);
        assert_eq!(bwt2, "PSSMIPISSII".as_bytes());
    }

    #[test]
    fn test_search_sa(){
        let s = ("MISSISSIPPI").to_string();
        let say = SA::<i32>::new(&s);
        let pat = ("ISS").to_string();
        let rst = say.search_sa(&pat.as_bytes());
        assert_eq!(rst, (2, 2));
    }
}
