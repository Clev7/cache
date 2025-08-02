pub mod replacement_policy;

use self::replacement_policy::ReplacementPolicy;

#[derive(Debug)]
struct Cache {
    block_size: u64,
    l1_size: u64,
    l1_assoc: u64,
    l2_size: u64,
    l2_assoc: u64,
    policy: ReplacementPolicy,
    is_inclusive: bool,
}