use crate::modules::target_iterators::cycle_group::cycle_group::cycle_group_ipv6_pattern::CycleIpv6Pattern;
use crate::modules::target_iterators::Ipv6Iter;

impl CycleIpv6Pattern {

    /// 以当前目标为基础, 计算并获取下一个目标
    /// 返回值: 0:是否为非最终值, 1:目标值
    #[inline]
    fn get_next_target(&mut self) -> (bool, u128) {

        loop {

            // (当前目标值 * 原根) % p
            self.current *= self.prim_root;
            self.current %= self.p;

            if self.current == self.last {
                // 如果当前乘法群的输出值为最终值, 标记为 false

                return if self.current < self.valid_range {
                    // 如果最终值合法, 返回最终值

                    // 注意: 这里对ip值合法性进行了检查
                    (false, self.current)
                } else {
                    (false, u128::MAX)
                }
            } else {

                // 使得 current 的值 始终处于  1..[    0..   |   tar_ip_num  ]
                // 注意这里 不等于0 的条件省略
                if self.current < self.valid_range {
                    return (true, self.current)
                }
            }
        }
    }

    #[inline]
    fn get_real_ip_from_tar_val(&self, ip_val: u128) -> u128 {
        // [  0..  ( 位数 : 128 - bits_for_ip)  |    part1 ( 位数: parts.0 )   |   part2 ( 位数: parts.0 )  |  part3 ( 位数: parts.0 ) ]
        // =>
        // 清除前置比特位
        // [ part1 |   0..    (128 - parts.0)      ]
        // [ part2 |   0..    (128 - parts.0)      ]
        // [ part3 |   0..    (128 - parts.0)      ]
        // =>
        // 清除后置比特位
        // [ 0..  | part1 ]
        // [ 0..  | part2 ]
        // [ 0..  | part3 ]
        // =>
        // 使用偏移量进行调整
        // [ part1  |       0..   ( 位数: parts.1 )                   ]
        // [    0..         |  part2 |     0..  ( 位数: parts.1 )     ]
        // [    0..                         |  part3 ]      // ( 位数: parts.1  为 0 )
        // =>
        // 所有片段 或运算
        // [  part1 |  0..  |  part2 |  0.. |  part3 ]


        // 对所有ip片段进行 或运算
        let mut real_ip = self.base_ip_val;
        for part_move in self.ip_move_len.iter() {
            // (0: 第一次左移位数, 1: 右移位数, 2: 第二次左移位数)

            let cur_part =
                ((ip_val << part_move.0) >> part_move.1) << part_move.2;
            real_ip = real_ip | cur_part;
        }

        real_ip
    }
}

impl Ipv6Iter for CycleIpv6Pattern {
    fn get_first_ip(&mut self) -> (bool, bool, u128) {
        if self.current == self.last {
            // 如果初始值是最后一个

            if self.current < self.valid_range {
                // ip值有效

                // ip值有效, 得到的 真实ip 也一定有效
                // 注意: 这里ip值需要减一
                let real_ip = self.get_real_ip_from_tar_val(self.current-1);
                (false, true, real_ip)
            } else {
                // 如果 超出有效范围
                (false, false, 0)
            }
        } else {
            // 如果初始值不是最后一个

            if self.current < self.valid_range {
                // ip值有效

                // ip值有效, 得到的 真实ip 也一定有效
                // 注意: 这里ip值需要减一
                let real_ip = self.get_real_ip_from_tar_val(self.current-1);
                (true, false, real_ip)
            } else {
                // ip值 超出有效范围
                self.get_next_ip()
            }
        }
    }

    fn get_next_ip(&mut self) -> (bool, bool, u128) {
        let target_ip_val = self.get_next_target();

        if target_ip_val.0 {
            // 如果不是最终值

            // ip值有效, 得到的 真实ip 也一定有效
            // 注意: 这里ip值需要减一
            let real_ip = self.get_real_ip_from_tar_val(self.current-1);
            (true, false, real_ip)
        } else {
            // 如果是最终值
            if target_ip_val.1 == u128::MAX {
                // 如果最终值无效
                (false, false, 0)
            } else {
                // 如果最终值有效

                // ip值有效, 得到的 真实ip 也一定有效
                // 注意: 这里ip值需要减一
                let real_ip = self.get_real_ip_from_tar_val(self.current-1);
                (false, true, real_ip)
            }
        }
    }
}