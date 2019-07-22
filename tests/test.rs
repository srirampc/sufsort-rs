#[cfg(test)]
mod tests {
    extern crate sufsort_rs as ss;
    use self::ss::sufsort::SA;
    use self::ss::sufsort::BWT;
    use self::ss::sufsort::construct_bwt_sa;
    use self::ss::rmq::RMQ;
    use self::ss::rmq::find_min_element;

    extern crate rand;
    use self::rand::Rng;

    #[test]
    fn test_sufsort(){
        let txt = ("MISSISSIPPI").to_string();
        let s = txt.as_bytes();
        {
          let say = SA::<i32>::new(&s);
          assert_eq!(say.sarray, &[10, 7, 4, 1, 0, 9, 8, 6, 3, 5, 2]);
        }
    }

    #[test]
    fn test_bwt(){
        let txt = ("MISSISSIPPI").to_string();
        let s = txt.as_bytes();
        {
            let bwx = BWT::<i32>::new(&s);
            assert_eq!(bwx.bwt, "PSSMIPISSII".as_bytes());
        }
    }

    #[test]
    fn test_bwt_sa(){
        let txt = ("MISSISSIPPI").to_string();
        let s = txt.as_bytes();
        let mut say = SA::<i32>::new(&s);
        let bwt2 = construct_bwt_sa(s, &mut say.sarray);
        assert_eq!(bwt2, "PSSMIPISSII".as_bytes());
    }

    #[test]
    fn test_search_sa(){
        let txt = ("MISSISSIPPI").to_string();
        let s = txt.as_bytes();
        let say = SA::<i32>::new(&s);
        let pat = ("ISS").to_string();
        let rst = say.search_sa(&pat.as_bytes());
        assert_eq!(rst, (2, 2));
    }

    pub struct RmqTester<'s, ST, IT> where 
        ST: std::marker::Copy + std::cmp::Ord + std::fmt::Debug,
        IT: std::marker::Copy + num::Integer + num::Unsigned +
            num::FromPrimitive + num::ToPrimitive + std::fmt::Debug {
        pub src:&'s [ST],
        pub r: RMQ<'s, ST, IT>,
    }

    impl<'s, ST, IT> RmqTester<'s,  ST, IT> where 
        ST: std::marker::Copy + std::cmp::Ord + std::fmt::Debug,
        IT: std::marker::Copy + num::Integer + num::Unsigned +
            num::FromPrimitive + num::ToPrimitive + std::fmt::Debug {

        pub fn new(source:&'s [ST]) -> Self {
            let trmq : RMQ<ST, IT> = RMQ::new(source);
            RmqTester{src: source, r: trmq}
        }

        pub fn check_superblock_correctness(&self) -> bool {
            // for (index_t d = 0; d < this->superblock_mins.size(); ++d) {
            for d in 0..self.r.superblock_mins.len(){
                // // checking superblock correctness for block `d`
                // index_t dist = 1<<d;
                let dist: usize = 1 << d;
                // ASSERT_EQ(this->n_superblocks - dist/2, this->superblock_mins[d].size()) << "Superblock minimum size is wrong for d=" << d;
                assert_eq!(self.r.n_superblocks - dist/2, self.r.superblock_mins[d].len(),
                            "Superblock minimum size is wrong for d={}", d);
                // for (index_t i = 0; i < this->superblock_mins[d].size(); ++i) {
                for i in 0..self.r.superblock_mins[d].len(){
                    // Iterator minel_pos = std::min_element(this->_begin + i*base_class::SUPERBLOCK_SIZE, 
                    //                                       std::min(this->_begin + (i+dist)*base_class::SUPERBLOCK_SIZE, this->_end));
                    let sdx = i * RMQ::<ST, IT>::SUPERBLOCK_SIZE;
                    let edx = std::cmp::min((i+dist)*RMQ::<ST, IT>::SUPERBLOCK_SIZE,
                                            self.src.len());
                    let minel_pos = find_min_element(self.src, sdx, edx);
                    // ASSERT_EQ(*minel_pos, 
                    //   *(this->_begin + this->superblock_mins[d][i])) <<
                    // "Superblock min is wrong for indeces: [d],[i]=" << d << "," << i;
                    let sbmx = self.r.superblock_mins[d][i].to_usize().unwrap();
                    assert_eq!(self.src[minel_pos], self.src[sbmx],
                               "Superblock min is wrong for indeces: [d],[i]={} {} {} {} {} {}", d , i, sdx, edx, minel_pos, sbmx);

                // }
                }
            // }
            }
            true
        }

        pub fn check_block_correctness(&self) -> bool {
            // for (index_t d = 0; d < this->block_mins.size(); ++d) {
            for d in 0..self.r.block_mins.len(){
                // // checking block correctness for block `d`
                // index_t dist = 1<<d;
                let dist: usize = 1 << d;
                // //assert(block_mins[d].size() == n_blocks - (n_superblocks)dist/2);
                // assert_eq!(self.r.block_mins[d].len(), n_blocks - (n_superblocks)dist/2);
                println!("BMN {} {}", self.r.block_mins[d].len(), RMQ::<ST, IT>::BLOCK_SIZE);
                // for (index_t i = 0; i < this->block_mins[d].size(); ++i) {
                for i in 0..self.r.block_mins[d].len(){
                    // index_t n_sb = i / (base_class::NB_PER_SB - dist/2);
                    let n_sb = i / (RMQ::<ST, IT>::NB_PER_SB - dist/2);
                    // index_t block_sb_idx = i % (base_class::NB_PER_SB - dist/2);
                    let block_sb_idx = i % (RMQ::<ST, IT>::NB_PER_SB - dist/2);
                    // index_t block_idx = base_class::NB_PER_SB*n_sb + block_sb_idx;
                    let block_idx = RMQ::<ST, IT>::NB_PER_SB * n_sb + block_sb_idx;
                    // index_t sb_end = base_class::SUPERBLOCK_SIZE*(n_sb+1);
                    let sb_end = RMQ::<ST, IT>::SUPERBLOCK_SIZE*(n_sb+1);
                    // Iterator minel_pos = std::min_element(
                    //     this->_begin + block_idx*base_class::BLOCK_SIZE, 
                    //     std::min(this->_begin + (block_idx+dist)*base_class::BLOCK_SIZE, 
                    //              std::min(this->_begin+sb_end,this->_end)));
                    let sdx = block_idx * RMQ::<ST, IT>::BLOCK_SIZE;
                    let edx = std::cmp::min((block_idx + dist) * RMQ::<ST, IT>::BLOCK_SIZE, 
                                            std::cmp::min(sb_end, self.src.len()));
                    let minel_pos = find_min_element(self.src, sdx, edx);
                    // //index_t minel_idx = minel_pos - this->_begin;
                    // //index_t rmq_idx = base_class::SUPERBLOCK_SIZE*n_sb + this->block_mins[d][i];
                    // ASSERT_EQ(*minel_pos, *(this->_begin + base_class::SUPERBLOCK_SIZE*n_sb + this->block_mins[d][i]));
                    assert!(minel_pos < self.src.len());
                    assert_eq!(self.src[minel_pos], 
                        self.src[RMQ::<ST, IT>::SUPERBLOCK_SIZE*n_sb + 
                            (self.r.block_mins[d][i] as usize)]);
                // }
                }
            // }
            }
            true
        }


        // in O(n^2)
        pub fn check_all_subranges(&self) -> bool {
            // size_t n = std::distance(this->_begin, this->_end);
            let n = self.src.len();

            // for (size_t i = 0; i < n-1; ++i) {
            for i in 0..(n-1) {
                // T min = *(this->_begin+i);
                let mut minx = self.src[i];
                let mut min_idx = i;
                // for (size_t j = i+1; j < n; ++j) {
                for j in i+1..n {
                    // if (*(this->_begin+j) < min)
                    //     min = *(this->_begin+j);
                    if self.src[j] < minx {
                        minx = self.src[j];
                        min_idx = j;
                    }
                    // ASSERT_EQ(min, *this->query(this->_begin+i, this->_begin+j+1)) 
                    //  << "wrong min for range (" << i << "," << j << ")";
                    let qx = self.r.query(i, j);
                    assert_eq!(minx, self.src[qx],
                        "wrong min for range {} {} {} {} {}", i, j, n, min_idx, qx);
                // }
                }
            // }
            }
            true
        }

        pub fn check_sub_range(&self, i : usize, j : usize) -> bool {
            // size_t n = std::distance(this->_begin, this->_end);
            let n = self.src.len();
            assert!(i < n && j <= n);
            assert!(i < j);

            let mut minx = self.src[i];
            let mut min_idx = i;
            for x in (i+1)..(j+1) {
                if self.src[x] < minx {
                    minx = self.src[x];
                    min_idx = x;
                }
            }
            let qx = self.r.query(i, j);
            assert_eq!(minx, self.src[qx],
               "wrong min for range {} {} {} {} {}", i, j, n, min_idx, qx);
            // println!("min for range {} {} {} {} {} {} {}", i, j, n, min_idx, qx, 
            //          self.src[min_idx].to_usize().unwrap(),
            //          self.src[qx].to_usize().unwrap());
            true
        }

    }

    #[test]
    fn test_rmq1(){
        let mut rng = rand::thread_rng();

        // for (size_t size : {1, 13, 32, 64, 127, 233}) {
        for n in &[1, 13, 32, 64, 127, 233] {
            // std::vector<int> vec(size);
            // std::generate(vec.begin(), vec.end(), [](){return std::rand() % 10;});
            let mut numbers = Vec::<i64>::with_capacity(*n as usize);
            for _ in 0..(*n as usize) {
                numbers.push(rng.gen::<i64>());
            }
            // // construct rmq
            // rmq_tester<std::vector<int>::iterator> r(vec.begin(), vec.end());
            let r : RmqTester<i64, usize> = RmqTester::<i64, usize>::new(&numbers);

            // // // check correctness
            r.check_block_correctness();
            r.check_superblock_correctness();
            r.check_all_subranges();
        // }
        }
    }
    
    #[test]
    fn test_rmq1_i32(){
        let mut rng = rand::thread_rng();

        // for (size_t size : {1, 13, 32, 64, 127, 233}) {
        for n in &[1, 13, 32, 64, 127, 233] {
            // std::vector<int> vec(size);
            // std::generate(vec.begin(), vec.end(), [](){return std::rand() % 10;});
            let mut numbers = Vec::<i32>::with_capacity(*n as usize);
            for _ in 0..(*n as usize) {
                numbers.push(rng.gen::<i32>());
            }
            // // construct rmq
            // rmq_tester<std::vector<int>::iterator> r(vec.begin(), vec.end());
            let r : RmqTester<i32, usize> = RmqTester::<i32, usize>::new(&numbers);

            // // // check correctness
            r.check_block_correctness();
            r.check_superblock_correctness();
            r.check_all_subranges();
        // }
        }
    }

    #[test]
    fn test_rmq2(){
        let mut rng = rand::thread_rng();
        // for (size_t size : {123, 73, 88, 1025}) {
        for n in &[123, 73, 88, 1025] {
            // std::vector<int> vec(size);
            // std::generate(vec.begin(), vec.end(), [](){return 50 - std::rand() % 100;});
            let mut numbers = Vec::<i64>::with_capacity(*n as usize);
            for _ in 0..(*n as usize) {
                numbers.push(50 - rng.gen::<i64>() % 100);
            }

            // // construct rmq
            // rmq_tester<std::vector<int>::iterator> r(vec.begin(), vec.end());
            let r : RmqTester<i64, usize> = RmqTester::<i64, usize>::new(&numbers);

            // // check correctness
            r.check_block_correctness();    
            r.check_superblock_correctness();
            if *n > 500 {
                r.check_sub_range(1, 254);
                r.check_sub_range(257, 500);
                r.check_sub_range(100, 300);
            }
            r.check_all_subranges();
        // }
        }
    }

    #[test]
    fn test_rmq_multiblocks(){
        let mut rng = rand::thread_rng();
        // for (size_t size : {123, 73, 88, 1024, 1033}) {
        for n in &[123, 73, 88, 1024, 1033] {
            // std::vector<unsigned int> vec(size);
            // std::generate(vec.begin(), vec.end(), [](){return std::rand() % 100;});
            let mut numbers = Vec::<u32>::with_capacity(*n as usize);
            for _ in 0..(*n as usize) {
                numbers.push(rng.gen::<u32>() % 100);
            }

            // // construct rmq
            // rmq_tester<std::vector<unsigned int>::iterator> r(vec.begin(), vec.end());
            let r : RmqTester<u32, usize> = RmqTester::<u32, usize>::new(&numbers);

            // // check correctness
            r.check_block_correctness();
            r.check_superblock_correctness();
            r.check_all_subranges();
        // }
        }
    }

    #[test]
    fn test_rmq_big(){
        // std::vector<size_t> vec(1235);
        let n : usize = 1235;
        let mut rng = rand::thread_rng();
        // std::generate(vec.begin(), vec.end(), [](){return std::rand() % 1000;});
        let mut numbers = Vec::<usize>::with_capacity(n);
        for _ in 0..(n) {
            numbers.push(rng.gen::<usize>());
        }
        // // construct rmq
        // rmq_tester<std::vector<size_t>::iterator> r(vec.begin(), vec.end());
        let r : RmqTester<usize, usize> = RmqTester::<usize, usize>::new(&numbers);
        // // check all queries
        r.check_block_correctness();
        r.check_superblock_correctness();
        // r.check_all_subranges();
    }

    #[test]
    fn test_rmq_multimin(){
        let mut rng = rand::thread_rng();
        // std::vector<size_t> vec(1000);
        // std::generate(vec.begin(), vec.end(), [](){return (8 + std::rand() % 10)/10;});
        // rmq<std::vector<size_t>::const_iterator> minquery(vec.cbegin(), vec.cend());
        let mut numbers = Vec::<usize>::with_capacity(1000);
        for _ in 0..1000 {
            numbers.push((8 + rng.gen::<usize>() % 10)/10);
        }
        let r: RMQ<usize, usize> = RMQ::<usize, usize>::new(&numbers);

        // // check whether the min is the first min in the range
        // // TODO: test for all partial ranges
        // auto begin = vec.cbegin();
        // auto min_it = minquery.query(vec.cbegin(), vec.cend());
        let mut begin = 0;
        let mut min_it = r.query(0, numbers.len()-1);
        // while (*min_it == 0) {
        while min_it < numbers.len() && numbers[min_it] == 0 {
            // if (min_it - begin > 0) {
            //     // assert the minimum of the range prior to the found min is larger
            //     auto min_it2 = minquery.query(begin, min_it);
            //     EXPECT_LT(*min_it, *min_it2) << " min for range [" << (begin-vec.cbegin()) << ",end] at pos "
            //  << (min_it - vec.cbegin()) 
            // << ", but there is a previous min of same value at pos " << (min_it2 - vec.cbegin());
            // }
            if min_it - begin > 0 {
                let min_it2 = r.query(begin, min_it - 1);
                assert!(numbers[min_it] < numbers[min_it2],  
                    " min for range [{}, end] at pos {}, but there is prev min of same val at pos {}  ",
                     begin, min_it, min_it2);
            }
            // // continue in remaining range:
            // begin = min_it+1;
            begin = min_it+1;
            // if (begin == vec.cend())
            //     break;
            if begin == numbers.len(){
                break;
            }
            // min_it = minquery.query(begin, vec.cend());
            min_it = r.query(begin, numbers.len() - 1)
        // }
        }
        // //std::cout << min_it - vec.cbegin() << std::endl;
    }

}
