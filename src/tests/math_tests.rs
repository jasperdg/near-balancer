use super::*;
use crate::math;

#[test]
fn test_pow() {
    let pow_of_2 = math::pow_u128(2, 2);
    
    assert_eq!(pow_of_2, 4);
}