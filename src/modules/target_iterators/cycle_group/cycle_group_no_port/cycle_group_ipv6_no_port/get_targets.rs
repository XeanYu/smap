use crate::modules::target_iterators::cycle_group::cycle_group_no_port::cycle_group_ipv6_no_port::CycleIpv6NoPort;

impl CycleIpv6NoPort {

    /// 以当前目标为基础, 计算并获取下一个目标
    /// 返回值: 0:是否为非最终值, 1:目标值
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


    /// 获取第一个目标的 ip
    /// 返回值: 0:是否为<u>非最终值</u>, 1:最终值是否有效, 2:ip地址
    pub fn get_first_ip_port(&mut self) -> (bool, bool, u128) {

        if self.current == self.last {
            // 如果初始值是最后一个

            if self.current < self.valid_range {
                // ip值有效

                // ip值有效, 得到的 真实ip 也一定有效
                let real_ip = self.start_ip + (self.current - 1);
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
                let real_ip = self.start_ip + (self.current - 1);
                (true, false, real_ip)
            } else {
                // ip值 超出有效范围
                self.get_next_ip_port()
            }
        }
    }

    /// 获取下一个目标的 ip
    /// 返回值: 0:是否为<u>非最终值</u>, 1:最终值是否有效, 2:ip地址
    pub fn get_next_ip_port(&mut self) -> (bool, bool, u128) {

        let target_ip_val = self.get_next_target();

        if target_ip_val.0 {
            // 如果不是最终值

            // ip值有效, 得到的 真实ip 也一定有效
            let real_ip = self.start_ip + (self.current - 1);
            (true, false, real_ip)
        } else {
            // 如果是最终值
            if target_ip_val.1 == u128::MAX {
                // 如果最终值无效
                (false, false, 0)
            } else {
                // 如果最终值有效

                // ip值有效, 得到的 真实ip 也一定有效
                let real_ip = self.start_ip + (self.current - 1);
                (false, true, real_ip)
            }
        }
    }
}
