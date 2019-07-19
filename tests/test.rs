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

    pub struct RmqTester<'s, T> where 
        T: std::clone::Clone + std::marker::Copy +
            std::ops::Add +
            std::cmp::Ord +
            num::FromPrimitive + num::ToPrimitive +
            num::One + num::Zero + std::fmt::Debug {
        pub src:&'s [T],
        pub r: RMQ<'s, T>,
    }

    impl<'s, T> RmqTester<'s, T> where
        T: std::clone::Clone + std::marker::Copy + std::ops::Add + std::cmp::Ord +
            num::FromPrimitive + num::ToPrimitive + num::One + num::Zero +
            std::fmt::Debug {

        pub fn new(source:&'s [T]) -> Self {
            let trmq : RMQ<T> = RMQ::new(source);
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
                    let sdx = i * RMQ::<T>::SUPERBLOCK_SIZE;
                    let edx = std::cmp::min((i+dist)*RMQ::<T>::SUPERBLOCK_SIZE,
                                            self.src.len());
                    let minel_pos = find_min_element(self.src, sdx, edx);
                    // ASSERT_EQ(*minel_pos, 
                    //   *(this->_begin + this->superblock_mins[d][i])) <<
                    // "Superblock min is wrong for indeces: [d],[i]=" << d << "," << i;
                    assert_eq!(self.src[minel_pos], self.src[self.r.superblock_mins[d][i].to_usize().unwrap()]);

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
                // for (index_t i = 0; i < this->block_mins[d].size(); ++i) {
                for i in 0..self.r.block_mins[d].len(){
                    // index_t n_sb = i / (base_class::NB_PER_SB - dist/2);
                    let n_sb = i / (RMQ::<T>::NB_PER_SB - dist/2);
                    // index_t block_sb_idx = i % (base_class::NB_PER_SB - dist/2);
                    let block_sb_idx = i % (RMQ::<T>::NB_PER_SB - dist/2);
                    // index_t block_idx = base_class::NB_PER_SB*n_sb + block_sb_idx;
                    let block_idx = RMQ::<T>::NB_PER_SB * n_sb + block_sb_idx;
                    // index_t sb_end = base_class::SUPERBLOCK_SIZE*(n_sb+1);
                    let sb_end = RMQ::<T>::SUPERBLOCK_SIZE*(n_sb+1);
                    // Iterator minel_pos = std::min_element(
                    //     this->_begin + block_idx*base_class::BLOCK_SIZE, 
                    //     std::min(this->_begin + (block_idx+dist)*base_class::BLOCK_SIZE, 
                    //              std::min(this->_begin+sb_end,this->_end)));
                    let sdx = block_idx * RMQ::<T>::BLOCK_SIZE;
                    let edx = std::cmp::min((block_idx + dist)* RMQ::<T>::BLOCK_SIZE, 
                                            std::cmp::min(sb_end, self.src.len()));
                    let minel_pos = find_min_element(self.src, sdx, edx);
                    // //index_t minel_idx = minel_pos - this->_begin;
                    // //index_t rmq_idx = base_class::SUPERBLOCK_SIZE*n_sb + this->block_mins[d][i];
                    // ASSERT_EQ(*minel_pos, *(this->_begin + base_class::SUPERBLOCK_SIZE*n_sb + this->block_mins[d][i]));
                    assert_eq!(self.src[minel_pos], self.src[RMQ::<T>::SUPERBLOCK_SIZE*n_sb + (self.r.block_mins[d][i] as usize)]);
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
                // for (size_t j = i+1; j < n; ++j) {
                for j in i+1..n {
                    // if (*(this->_begin+j) < min)
                    //     min = *(this->_begin+j);
                    if(self.src[j] < minx){
                        minx = self.src[j]
                    }
                    // ASSERT_EQ(min, *this->query(this->_begin+i, this->_begin+j+1)) 
                    //  << "wrong min for range (" << i << "," << j << ")";
                    assert_eq!(minx, self.src[self.r.query(i, j+1)],
                        "wrong min for range {} {} {}", i, j, n);
                // }
                }
            // }
            }
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
            let r : RmqTester<i64> = RmqTester::<i64>::new(&numbers);

            // // check correctness
            // r.check_block_correctness();
            assert!(r.check_block_correctness(), true);
            assert!(r.check_superblock_correctness(), true);
            assert!(r.check_all_subranges(), true);
        // }
        }
    }
    
    #[test]
    fn test_rmq2(){
    // for (size_t size : {123, 73, 88, 1025}) {
    //     std::vector<int> vec(size);
    //     std::generate(vec.begin(), vec.end(), [](){return 50 - std::rand() % 100;});
    //     // construct rmq
    //     rmq_tester<std::vector<int>::iterator> r(vec.begin(), vec.end());

    //     // check correctness
    //     r.check_block_correctness();
    //     r.check_superblock_correctness();
    //     r.check_all_subranges();
    // }
    }

    #[test]
    fn test_rmq_multiblocks(){
        // for (size_t size : {123, 73, 88, 1024, 1033}) {
        //     std::vector<unsigned int> vec(size);
        //     std::generate(vec.begin(), vec.end(), [](){return std::rand() % 100;});
        //     // construct rmq
        //     rmq_tester<std::vector<unsigned int>::iterator> r(vec.begin(), vec.end());

        //     // check correctness
        //     r.check_block_correctness();
        //     r.check_superblock_correctness();
        //     r.check_all_subranges();
        // }
    }

    #[test]
    fn test_rmq_big(){
        // std::vector<size_t> vec(1235);
        // std::generate(vec.begin(), vec.end(), [](){return std::rand() % 1000;});
        // // construct rmq
        // rmq_tester<std::vector<size_t>::iterator> r(vec.begin(), vec.end());
        // // check all queries
        // r.check_all_subranges();
    }

    #[test]
    fn test_rmq_multimin(){

    // std::vector<size_t> vec(1000);
    // std::generate(vec.begin(), vec.end(), [](){return (8 + std::rand() % 10)/10;});
    // rmq<std::vector<size_t>::const_iterator> minquery(vec.cbegin(), vec.cend());

    // // check whether the min is the first min in the range
    // // TODO: test for all partial ranges
    // auto begin = vec.cbegin();
    // auto min_it = minquery.query(vec.cbegin(), vec.cend());
    // while (*min_it == 0) {
    //     if (min_it - begin > 0) {
    //         // assert the minimum of the range prior to the found min is larger
    //         auto min_it2 = minquery.query(begin, min_it);
    //         EXPECT_LT(*min_it, *min_it2) << " min for range [" << (begin-vec.cbegin()) << ",end] at pos " << (min_it - vec.cbegin()) << ", but there is a previous min of same value at pos " << (min_it2 - vec.cbegin());
    //     }
    //     // continue in remaining range:
    //     begin = min_it+1;
    //     if (begin == vec.cend())
    //         break;
    //     min_it = minquery.query(begin, vec.cend());
    // }
    // //std::cout << min_it - vec.cbegin() << std::endl;
    }

}
