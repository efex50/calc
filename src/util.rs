


pub fn as_i128<S:Into<String>>(a:S) -> Result<i128,()>{
    let a:String = a.into();
    let num :Result<i128, std::num::ParseIntError>;
    if a.contains("0b"){
        let without_prefix = a.trim_start_matches("0b");
        num = i128::from_str_radix(without_prefix, 2);
    }
    else if a.contains("0x"){
        let without_prefix = a.trim_start_matches("0x");
        num = i128::from_str_radix(without_prefix, 16);
    }
    else if a.contains("0o"){
        let without_prefix = a.trim_start_matches("0o");
        num = i128::from_str_radix(without_prefix, 8);
    }
    else {
        num = i128::from_str_radix(&a, 10);
    };
    num
    .map(|a|Ok(a))
    .unwrap_or(Err(()))
    
}
