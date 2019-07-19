
pub fn find_min_element<T>(src:& [T], start: usize, end: usize) -> usize
        where
        T: std::cmp::Ord  {
    if start > src.len() || start >= end {
        src.len()
    } 
    else if src.len() <= 1 {
        start
    } else {
        let mut mpos = start;
        for sx in (start+1)..std::cmp::min(end, src.len()) {
            if sx >= src.len() {
                break;
            }
            if src[sx] < src[mpos] {
                mpos = sx;
            }
        }
        mpos
    }
}


pub fn log2_usize(vxl: usize) -> usize {
    const tab64 : [usize; 64] = [
        63,  0, 58,  1, 59, 47, 53,  2,
        60, 39, 48, 27, 54, 33, 42,  3,
        61, 51, 37, 40, 49, 18, 28, 20,
        55, 30, 34, 11, 43, 14, 22,  4,
        62, 57, 46, 52, 38, 26, 32, 41,
        50, 36, 17, 19, 29, 10, 13, 21,
        56, 45, 25, 31, 35, 16,  9, 12,
        44, 24, 15,  8, 23,  7,  6,  5
    ];

    let mut value = vxl;
    value |= value >> 1;
    value |= value >> 2;
    value |= value >> 4;
    value |= value >> 8;
    value |= value >> 16;
    value |= value >> 32;
    tab64[((value - (value >> 1))*0x07EDD5E59A4E28C2usize) >> 58]
}

pub fn reference_ceil_log2(x: usize) -> usize {
    let mut log_floor: usize = 0;
    let mut n: usize = x;
    // for (;n != 0; n >>= 1)
    while n != 0 {
        log_floor += 1;
        n >>= 1;
    }
    log_floor -= 1;
    // add one if not power of 2
    log_floor + if (x&(x-1)) != 0 { 1 } else { 0 }
}


pub fn floor_log2(n: usize) -> usize {
    //return log2_64(n);
    log2_usize(n)
}

pub fn ceil_log2(n: usize) -> usize {
    let log_floor = floor_log2(n);
    // add one if not power of 2
    log_floor + if (n & (n-1)) != 0 { 1 } else { 0 }
}




pub struct RMQ<'s, T> where 
    T: std::clone::Clone + std::marker::Copy + std::ops::Add + std::cmp::Ord +
        num::FromPrimitive + num::ToPrimitive + num::One + num::Zero {

    pub src:&'s [T],

    pub n: usize,

    pub n_blocks: usize,

    pub n_superblocks: usize,

    // saves minimum as block index for combination of superblocks
    // relative to global start index
    pub superblock_mins: Vec<Vec<T>>,

    // saves minimum for combination of blocks relative to superblock
    // start index
    pub block_mins: Vec<Vec<u16>>,

}


impl<'s, T: 's> RMQ<'s, T> where
    T: std::clone::Clone + std::marker::Copy + std::ops::Add + std::cmp::Ord +
        num::FromPrimitive + num::ToPrimitive + num::One + num::Zero {

    // superblock size is log^(2+epsilon)(n)
    // we choose it as bitsize^2:
    //  - 64 bits -> 4096
    //  - 32 bits -> 1024
    //  - both bound by 2^16 -> uint16_t
    //  block size: one cache line
    pub const BLOCK_SIZE: usize = 64/std::mem::size_of::<T>();
    pub const SUPERBLOCK_SIZE: usize = 64/std::mem::size_of::<u16>() * 64/std::mem::size_of::<T>();
    pub const NB_PER_SB: usize = Self::SUPERBLOCK_SIZE / Self::BLOCK_SIZE;

    // static_assert(sizeof(index_t) == 8 || sizeof(index_t) == 4,
    // "TODO: RMQ implemented only for sizeof(index_t) = 8 or 4");
    pub const LOG_IDXT : usize = 3; // TODO: fix this
    // if std::mem::size_of::<T>() == 8 { 3 } else { 4};
    pub const LOG_B_SIZE : usize = 6 - Self::LOG_IDXT;
    pub const LOG_SB_SIZE : usize = 12 - Self::LOG_IDXT - 1;
    pub const LOG_NB_PER_SB : usize = Self::LOG_SB_SIZE - Self::LOG_B_SIZE;

    pub fn new(source: &'s [T]) -> RMQ<'s, T> {
        let n = source.len();
        // // get number of blocks
        // n_superblocks = ((n-1) >> LOG_SB_SIZE) + 1;
        let n_superblocks = ((n-1) >> Self::LOG_SB_SIZE) + 1;
        // n_blocks = ((n-1) >> LOG_B_SIZE) + 1;
        let n_blocks = ((n-1) >> Self::LOG_B_SIZE) + 1;

        // superblock_mins.push_back(std::vector<index_t>(n_superblocks));
        // block_mins.push_back(std::vector<uint16_t>(n_blocks));
        let mut superblock_mins : Vec<Vec<T>> = Vec::new(); // = Vec::with_capacity(n_superblocks);
        let mut block_mins : Vec<Vec<u16>> = Vec::new();
        superblock_mins.push(vec![T::zero(); n_superblocks]);
        block_mins.push(vec![0; n_blocks]);
        let mut it_idx: usize = 0;
        let it_end = n;
       //  Iterator it = begin;
       //  while (it != end) {
        while it_idx < it_end {
            // // find index of minimum block in superblock
            // Iterator min_pos = it;
            let mut min_pos = it_idx;
            // Iterator sb_end_it = it;
            // std::advance(sb_end_it, std::min<std::size_t>(std::distance(it, end), SUPERBLOCK_SIZE));
            let it_sb_end = it_idx + std::cmp::min(
                std::cmp::min(it_end - it_idx, Self::SUPERBLOCK_SIZE), n);

            // for (Iterator block_it = it; block_it != sb_end_it; ) {
            let mut it_block = it_idx;
            while it_block < it_sb_end {
                // Iterator block_end_it = block_it;
                // std::advance(block_end_it, std::min<std::size_t>(std::distance(block_it, end), BLOCK_SIZE));
                let it_block_end = it_block + std::cmp::min(
                    std::cmp::min(it_end - it_block, Self::BLOCK_SIZE), n); 

                // Iterator block_min_pos = std::min_element(block_it, block_end_it);
                // // save minimum for superblock min
                // if (*block_min_pos < *min_pos) {
                //     min_pos = block_min_pos;
                // }
                let block_min_pos : usize  = find_min_element(source, it_block, it_block_end);
                if block_min_pos < it_block_end && source[block_min_pos] < source[min_pos]{
                    min_pos = block_min_pos
                }
                // // save minimum for block min, relative to superblock start
                // index_t block_min_idx = static_cast<index_t>(std::distance(it, block_min_pos));
                // assert(block_min_idx < SUPERBLOCK_SIZE);
                let block_min_idx : T = T::from_usize(block_min_pos).unwrap();
                
                // block_mins[0][std::distance(begin, block_it) >> LOG_B_SIZE] = static_cast<uint16_t>(block_min_idx);
                block_mins[0][it_block >> Self::LOG_B_SIZE] = block_min_idx.to_u16().unwrap();
                // block_it = block_end_it;
                it_block = it_block_end
            // }
            }
            // superblock_mins[0][std::distance(begin, it) >> LOG_SB_SIZE] = static_cast<index_t>(std::distance(begin, min_pos));
            let sbmin_idx = it_idx >> Self::LOG_B_SIZE;
            superblock_mins[0][sbmin_idx] =  T::from_usize(min_pos).unwrap();
            // it = sb_end_it;
            it_idx = it_sb_end;
        // }
        }


        // fill superblock lookup with dynamic programming
        // index_t level = 1;
        let mut level: usize = 1;
        // for (index_t dist = 2; dist/2 < n_superblocks; dist <<= 1) {
        let mut dist: usize = 2;
        while dist/2 < n_superblocks {
            // superblock_mins.push_back(std::vector<index_t>(n_superblocks - dist/2));
            superblock_mins.push(vec![T::zero(); n_superblocks - dist/2]);
            // for (index_t i = 0; i+dist/2 < n_superblocks; ++i) {
            let mut idx = 0;
            while idx + (dist/2) < n_superblocks {
                // index_t right_idx = std::min(i+dist/2, n_superblocks-dist/4-1);
                let right_idx: usize = std::cmp::min(idx + dist/2, n_superblocks - dist/4 - 1);
                // if (*(begin + superblock_mins[level-1][right_idx]) < *(begin + superblock_mins[level-1][i])) {
                if source[superblock_mins[level-1][right_idx].to_usize().unwrap()] < 
                    source[superblock_mins[level-1][right_idx].to_usize().unwrap()] {
                    // assert(i < superblock_mins.back().size());
                    assert!(idx < superblock_mins.last().unwrap().len());
                    // assert(superblock_mins.size() == level+1);
                    assert!(superblock_mins.len() == level+1);
                    // assert(superblock_mins[level-1].size() > right_idx);
                    assert!(superblock_mins[level-1].len() > right_idx);
                    // superblock_mins.back()[i] = superblock_mins[level-1][right_idx];
                    superblock_mins.last_mut().unwrap()[idx] = superblock_mins[level-1][right_idx];
                // } else {
                } else {
                    // assert(i < superblock_mins.back().size());
                    assert!(idx < superblock_mins.last().unwrap().len());
                    // assert(superblock_mins.size() == level+1);
                    assert!(superblock_mins.len() == level+1);
                    // superblock_mins.back()[i] = superblock_mins[level-1][i];
                    superblock_mins.last_mut().unwrap()[idx] = superblock_mins[level-1][idx];
                // }
                }
                idx = idx + 1;
            // }
            }
            // level++;
            level = level + 1;
            dist <<= 1;
        // }
        }

        // now the same thing for blocks (but index relative to their
        // superblock)
        // level = 1;
        level = 1;
        // index_t last_sb_nblocks = n_blocks - ((n_superblocks-1) << LOG_NB_PER_SB);
        let last_sb_nblocks: usize = n_blocks - ((n_superblocks-1) << Self::LOG_NB_PER_SB);
        // for (index_t dist = 2; dist/2 < std::min<size_t>(n_blocks,NB_PER_SB); dist <<= 1) {
        dist = 2;
        while dist/2 < std::cmp::min(n_blocks, Self::NB_PER_SB) {
            // if (n_blocks - n_superblocks*dist/2 == 0)
            //     break;
            if n_blocks - n_superblocks * dist/2 == 0 {
                break;
            }
            // index_t last_sb_cur_nblocks = 0;
            let mut last_sb_cur_nblocks: usize = 0;
            // if (last_sb_nblocks > dist/2)
            //     last_sb_cur_nblocks = last_sb_nblocks - dist/2;
            if last_sb_nblocks > dist/2 {
                last_sb_cur_nblocks = last_sb_nblocks - dist/2;
            }
            // block_mins.push_back(std::vector<uint16_t>((n_superblocks-1)*(NB_PER_SB - dist/2) + last_sb_cur_nblocks));
            block_mins.push(vec![0; (n_superblocks-1)*(Self::NB_PER_SB - dist/2) + last_sb_cur_nblocks]);
            // for (index_t sb = 0; sb < n_superblocks; ++sb) {
            for sb in 0..n_superblocks {
                // index_t pre_sb_offset = sb*(NB_PER_SB - dist/4);
                let pre_sb_offset = sb*(Self::NB_PER_SB - dist/4);
                // index_t sb_offset = sb*(NB_PER_SB - dist/2);
                let sb_offset = sb*(Self::NB_PER_SB - dist/2);
                // index_t blocks_in_sb = std::min<size_t>(n_blocks - sb*NB_PER_SB, NB_PER_SB);
                let blocsk_in_sb = std::cmp::min(n_blocks - sb*Self::NB_PER_SB, Self::NB_PER_SB);
                // for (index_t i = 0; i+dist/2 < blocks_in_sb; ++i) {
                let mut idx: usize = 0;
                while idx + dist/2 < blocsk_in_sb {
                    // // TODO: right_idx might become negative for last superblock??
                    // index_t right_idx = std::min(i + dist/2, blocks_in_sb-dist/4-1);
                    let right_idx = std::cmp::min(idx + dist/2, blocsk_in_sb - dist/4 - 1);
                    // if (*(begin + block_mins[level-1][right_idx+pre_sb_offset] + (sb << LOG_SB_SIZE))
                    //   < *(begin + block_mins[level-1][i        +pre_sb_offset] + (sb << LOG_SB_SIZE)))
                    // {
                    if  source[block_mins[level-1][right_idx + pre_sb_offset] as usize + (sb << Self::LOG_SB_SIZE)] <
                        source[block_mins[level-1][idx       + pre_sb_offset] as usize + (sb << Self::LOG_SB_SIZE)] {
                        // block_mins.back()[i+sb_offset] = block_mins[level-1][right_idx+pre_sb_offset];
                        block_mins.last_mut().unwrap()[idx+sb_offset] = block_mins[level-1][right_idx+pre_sb_offset];
                    // } else {
                    } else {
                        // block_mins.back()[i+sb_offset] = block_mins[level-1][i+pre_sb_offset];
                        block_mins.last_mut().unwrap()[idx+sb_offset] = block_mins[level-1][idx+pre_sb_offset];
                    // }
                    }
                    idx += 1;
                // }
                }
            // }
            }
            // level++;
            level = level + 1;
            dist <<= 1;
        // }
        }

        RMQ::<'s, T>{
            src: source, n: n,
            n_blocks: n_blocks,
            n_superblocks: n_superblocks,
            superblock_mins: superblock_mins,
            block_mins: block_mins,
        }
    }


//     /**
//      * @brief Query the inclusive query range [l,r]
//      *
//      * @return The index of the minimum item in the range [l,r]
//      */
//     size_t operator()(const size_t l, const size_t r) const {
       pub fn query(&self, l: usize, r: usize) -> usize {
        // const size_t begin_idx = l;
        let begin_idx = l;
        // const size_t end_idx = r+1;
        let end_idx = r+1;
        // assert(begin_idx < end_idx);
        assert!(begin_idx < end_idx);
        // assert(end_idx <= n);
        assert!(end_idx <= self.n);

        // // round up to next superblock
        // index_t left_sb  = ((begin_idx - 1) >> LOG_SB_SIZE) + 1;
        // if (begin_idx == 0)
        //     left_sb = 0;
        let left_sb = if begin_idx == 0 {
            0
        } else {
            ((begin_idx - 1) >> Self::LOG_SB_SIZE) + 1
        };
        // // round down to prev superblock
        // index_t right_sb = end_idx >> LOG_SB_SIZE;
        let right_sb = end_idx >> Self::LOG_SB_SIZE;

        // // init result
        // Iterator min_pos = _begin + begin_idx;
        let mut min_pos = begin_idx;

        // // if there is at least one superblock
        // if (left_sb < right_sb) {
        if left_sb < right_sb {
            // // get largest power of two that doesn't exceed the number of
            // // superblocks from (left,right)
            // index_t n_sb = right_sb - left_sb;
            let n_sb = right_sb - left_sb;
            // unsigned int dist = floorlog2(n_sb);
            let dist = log2_usize(n_sb);


            // assert(dist < superblock_mins.size() && left_sb < superblock_mins[dist].size());
            assert!(dist < self.superblock_mins.len() && left_sb < self.superblock_mins[dist].len());
            // min_pos = _begin + superblock_mins[dist][left_sb];
            min_pos = self.superblock_mins[dist][left_sb].to_usize().unwrap();
            // assert(dist < superblock_mins.size() && right_sb - (1<<dist) < superblock_mins[dist].size());
            assert!(dist < self.superblock_mins.len() && right_sb - (1<<dist) < self.superblock_mins[dist].len());
            // Iterator right_sb_min = _begin + superblock_mins[dist][right_sb - (1 << dist)];
            let right_sb_min = self.superblock_mins[dist][right_sb - (1 << dist)].to_usize().unwrap();
            // if (*right_sb_min < *min_pos) {
            //     min_pos = right_sb_min;
            // }
            if self.src[right_sb_min] < self.src[min_pos] {
                min_pos = right_sb_min;
            }
        // }
        }

        // // go to left -> blocks -> sub-block
        // if (left_sb <= right_sb && left_sb != 0 && begin_idx != (left_sb << LOG_SB_SIZE)) {
        if left_sb <= right_sb && left_sb != 0 && begin_idx != (left_sb << Self::LOG_SB_SIZE) {
            // index_t left_b = ((begin_idx - 1) >> LOG_B_SIZE) + 1;
            let mut left_b = ((begin_idx - 1) >> Self::LOG_B_SIZE) + 1;
            // index_t left_b_gidx = left_b << LOG_B_SIZE;
            let left_b_gidx = left_b << Self::LOG_B_SIZE;
            // left_b -= (left_sb - 1) << LOG_NB_PER_SB;
            left_b -= (left_sb - 1) << Self::LOG_NB_PER_SB;
            // index_t n_b = NB_PER_SB - left_b;
            let n_b = Self::NB_PER_SB - left_b;
            // if (n_b > 0) {
            if n_b > 0 {
                // unsigned int level = ceillog2(n_b);
                let level = ceil_log2(n_b);
                // index_t sb_offset = (left_sb-1)*(NB_PER_SB - (1<<level)/2);
                let sb_offset = (left_sb-1)*(Self::NB_PER_SB - (1<<level)/2);
                // Iterator block_min_it = _begin + block_mins[level][left_b + sb_offset] + ((left_sb-1)<<LOG_SB_SIZE);
                let block_min_it = self.block_mins[level][left_b + sb_offset] as usize + ((left_sb-1)<<Self::LOG_SB_SIZE);
                // // return this new min if its the same or smaller
                // if (!(*min_pos < *block_min_it))
                //     min_pos = block_min_it;
                if self.src[min_pos] < self.src[block_min_it] {
                    min_pos = block_min_it
                }
            // }
            }

            // // go left into remaining block, if elements left
            // if (left_b_gidx > begin_idx) {
            if left_b_gidx > begin_idx {
                // // linearly search (at most block_size elements)
                // Iterator inblock_min_it = std::min_element(_begin + begin_idx, _begin + left_b_gidx);
                let inblock_min_t = find_min_element(self.src, begin_idx, left_b_gidx);
                // if (!(*min_pos < *inblock_min_it)) {
                if self.src[min_pos] < self.src[inblock_min_t] {
                //     min_pos = inblock_min_it;
                    min_pos = inblock_min_t;
                // }
                }
            // }
            }
        // }
       }

        // // go to right -> blocks -> sub-block
        // if (left_sb <= right_sb && right_sb != n_superblocks && end_idx != (right_sb << LOG_SB_SIZE)) {
        if left_sb <= right_sb && right_sb != self.n_superblocks && end_idx != (right_sb << Self::LOG_SB_SIZE) {
            // index_t left_b = right_sb << LOG_NB_PER_SB;
            let left_b = right_sb << Self::LOG_NB_PER_SB;
            // index_t right_b = end_idx >> LOG_B_SIZE;
            let right_b = end_idx >> Self::LOG_B_SIZE;
            // index_t n_b = right_b - left_b;
            let n_b = right_b - left_b;
            // if (n_b > 0) {
            if n_b > 0 {
                // unsigned int dist = floorlog2(n_b);
                let dist = floor_log2(n_b);
                // index_t sb_offset = right_sb*((1<<dist)/2);
                let sb_offset = right_sb*((1<<dist)/2);
                // Iterator block_min_it = _begin + block_mins[dist][left_b - sb_offset] + ((right_sb) << LOG_SB_SIZE);
                let mut block_min_it = self.block_mins[dist][left_b - sb_offset] as usize + ((right_sb) << Self::LOG_SB_SIZE);
                // if (*block_min_it < *min_pos)
                //     min_pos = block_min_it;
                if self.src[block_min_it] < self.src[min_pos]{
                    min_pos = block_min_it
                }
                // block_min_it = _begin + block_mins[dist][right_b - sb_offset - (1<<dist)] + ((right_sb) << LOG_SB_SIZE);
                block_min_it = self.block_mins[dist][right_b - sb_offset - (1<<dist)] as usize + ((right_sb) << Self::LOG_SB_SIZE);
                // if (*block_min_it < *min_pos)
                //     min_pos = block_min_it;
                if self.src[block_min_it] < self.src[min_pos]{
                    min_pos = block_min_it;
                }
            // }
            }

            // // go right into remaining block, if elements left
            // index_t left_gl_idx = right_b << LOG_B_SIZE;
            let left_gl_idx = right_b << Self::LOG_B_SIZE;
            // if (left_gl_idx < end_idx) {
            if left_gl_idx < end_idx {
                // // linearly search (at most block_size elements)
                // Iterator inblock_min_it = std::min_element(_begin + left_gl_idx, _begin + end_idx);
                let inblock_min_it = find_min_element(self.src, left_gl_idx, end_idx);
                // if (*inblock_min_it < *min_pos) {
                //     min_pos = inblock_min_it;
                // }
                if self.src[inblock_min_it] < self.src[min_pos] {
                    min_pos = inblock_min_it;
                }
            // }
            }
        // }
        }

        // if there are no superblocks covered (both indeces in same superblock)
        // if (left_sb > right_sb) {
        if left_sb > right_sb {
            // index_t left_b = ((begin_idx - 1) >> LOG_B_SIZE) + 1;
            let mut left_b = ((begin_idx - 1) >> Self::LOG_B_SIZE) + 1;
            // if (begin_idx == 0)
            //     left_b = 0;
            if begin_idx == 0 { left_b = 0};
            // index_t right_b = end_idx >> LOG_B_SIZE;
            let right_b = end_idx >> Self::LOG_B_SIZE;


            // if (left_b < right_b) {
            if left_b < right_b {
                // if blocks are in between: get mins of blocks in range
                // NOTE: there was a while if-else block here to handle the
                //       case if blocks would span accross the boundary of two
                //       superblocks, this should however never happen
                //       git blame this line to find where this code was removed
                // assert blocks lie in the same superblock
                // assert(left_b >> LOG_NB_PER_SB == right_b >> LOG_NB_PER_SB);
                assert!(left_b >> Self::LOG_NB_PER_SB == right_b >> Self::LOG_NB_PER_SB);

                // unsigned int dist = floorlog2(right_b - left_b);
                let dist = floor_log2(right_b - left_b);
                // index_t sb_offset = 0;
                let mut sb_offset = 0;
                // index_t sb_size_offset = 0;
                let mut sb_size_offset = 0;
                // if (left_sb > 1) {
                //     sb_offset = (left_sb-1)*((1<<dist)/2);
                //     sb_size_offset = (left_sb-1) << LOG_SB_SIZE;
                // }
                if left_sb > 1 {
                    sb_offset = (left_sb-1)*((1<<dist)/2);
                    sb_size_offset = (left_sb-1) << Self::LOG_SB_SIZE;
                }
                // Iterator block_min_it = _begin + block_mins[dist][left_b - sb_offset] + sb_size_offset;
                let mut block_min_it = self.block_mins[dist][left_b - sb_offset] as usize + sb_size_offset;
                // if (*block_min_it < *min_pos)
                //     min_pos = block_min_it;
                if self.src[block_min_it] < self.src[min_pos] {
                    min_pos = block_min_it;
                }
                // block_min_it = _begin + block_mins[dist][right_b - sb_offset - (1<<dist)] + sb_size_offset;
                // if (*block_min_it < *min_pos)
                //     min_pos = block_min_it;
                block_min_it = self.block_mins[dist][right_b - sb_offset - (1<<dist)] as usize + sb_size_offset;
                // if (*block_min_it < *min_pos)
                //     min_pos = block_min_it;
                if self.src[block_min_it] < self.src[min_pos] {
                    min_pos = block_min_it;
                }

                // // remaining inblock
                // if (begin_idx < (left_b << LOG_B_SIZE)) {
                //     Iterator inblock_min_it = std::min_element(_begin + begin_idx, _begin + (left_b << LOG_B_SIZE));
                //     if (!(*min_pos < *inblock_min_it)) {
                //         min_pos = inblock_min_it;
                //     }
                // }
                if begin_idx < (left_b << Self::LOG_B_SIZE) {
                    let inblock_min_it = find_min_element(self.src, begin_idx, left_b << Self::LOG_B_SIZE);
                    if !(self.src[min_pos] < self.src[inblock_min_it]) {
                        min_pos = inblock_min_it;
                    }
                }
                // if (end_idx > (right_b << LOG_B_SIZE)) {
                //     Iterator inblock_min_it = std::min_element(_begin + (right_b << LOG_B_SIZE), _begin + end_idx);
                //     if (*inblock_min_it < *min_pos) {
                //         min_pos = inblock_min_it;
                //     }
                // }
                if end_idx > (right_b << Self::LOG_B_SIZE) {
                    let inblock_min_it = find_min_element(self.src, right_b << Self::LOG_B_SIZE, end_idx);
                    if self.src[inblock_min_it] < self.src[min_pos] {
                        min_pos = inblock_min_it;
                    }
                }
            // } else {
            } else {
                // // no blocks at all
                // Iterator inblock_min_it = std::min_element(_begin + begin_idx, _begin + end_idx);
                let inblock_min_it = find_min_element(self.src, begin_idx, end_idx);
                // if (*inblock_min_it < *min_pos) {
                //     min_pos = inblock_min_it;
                // }
                if self.src[inblock_min_it] < self.src[min_pos] {
                    min_pos = inblock_min_it;
                }
            // }
            }

        // }
        }

        // // return the minimum found
        // return min_pos - _begin;
        min_pos
    // }
    }
}

