use crate::constants::TOKEN_DENOM;
use uint::construct_uint;

construct_uint! {
    /// 256-bit unsigned integer.
    pub struct u256(4);
}

/**********************************************************************************************
// calcSpotPrice                                                                             //
// sP = spotPrice                                                                            //
// bI = tokenBalanceIn                ( bI / wI )         1                                  //
// bO = tokenBalanceOut         sP =  -----------  *  ----------                             //
// wI = tokenWeightIn                 ( bO / wO )     ( 1 - sF )                             //
// wO = tokenWeightOut                                                                       //
// sF = swapFee                                                                              //
**********************************************************************************************/

pub fn calc_spot_price(
    token_balance_in: u128,
    token_weight_in: u128,
    token_balance_out: u128,
    token_weight_out: u128,
    swap_fee: u128
) -> u128 {
    let numer = div_u128(token_balance_in, token_weight_in);
    let denom = div_u128(token_balance_out, token_weight_out);
    let ratio = div_u128(numer, denom);
    let scale = div_u128(TOKEN_DENOM, TOKEN_DENOM - swap_fee);

    div_u128(ratio, scale)
}

pub fn div_u128(a: u128, b: u128) -> u128 {
    let a_u256 = u256::from(a);
    let token_denom_u256 = u256::from(TOKEN_DENOM);

    let c0 = a_u256 * token_denom_u256;

    let c1 = c0 + (b / 2);

    (c1 / b).as_u128()
}