use crate::types::FixType;

pub fn as_i128<S:Into<String>>(a:S) -> Result<i128,()>{
    let a:String = a.into();
    let num :Result<i128, std::num::ParseIntError>;
    if a.starts_with("0b"){
        let without_prefix = a.trim_start_matches("0b");
        num = i128::from_str_radix(without_prefix, 2);
    }
    else if a.starts_with("0x"){
        let without_prefix = a.trim_start_matches("0x");
        num = i128::from_str_radix(without_prefix, 16);
    }
    else if a.starts_with("0o"){
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


pub(crate) fn _check_fix_type(str:&str)-> Result<FixType,()>{
    if str.is_empty(){
        return Err(());
    }
    
    let symbols = ["+","-","*","/",","];
    let chars:Vec<String> = split_str(&str, &symbols,&[',',' ']);

    if let Some(last) = chars.last(){
        if symbols.contains(&last.as_str()){
            if chars.len() > 1{
                return Ok(FixType::PostFıx);
            }
        }
    }
        
    let first = &chars[0];
    if symbols.contains(&first.as_str()){
        if first == "*" || first == "/" {
            return Ok(FixType::PreFıx);
        }
        if first == "-" || first == "+"{
            if chars.len() > 2 {
                let third = &chars[2];

                if !symbols.contains(&third.as_str()){
                    return Ok(FixType::PreFıx);
                }
            }else {
                return Ok(FixType::PreFıx);
            }
        }
    }
    for x in &chars {
        if symbols.contains(&x.as_str()) {
            return Ok(FixType::Infıx);
        };
    }

    Err(())
}


pub(crate) fn check_fix_type(str: &str) -> Result<FixType, ()> {
    if str.is_empty() {
        return Err(());
    }
        
    let symbols = ["+", "-", "*", "/", ",", "(", ")"];
    
    let chars: Vec<String> = split_str(&str, &symbols, &[',', ' ']);
    
    if chars.is_empty() { return Err(()); }
    if let Some(last) = chars.last() {
        if symbols.contains(&last.as_str()) && last != ")" {
            if chars.len() > 1 {
                return Ok(FixType::PostFıx);
            }
        }
    }

    let first = &chars[0];
    if symbols.contains(&first.as_str()) {
        if first == "(" {
            return Ok(FixType::Infıx);
        }

        if first == "*" || first == "/" {
            return Ok(FixType::PreFıx);
        }

        if first == "-" || first == "+" {
            if chars.len() > 1 {
                let second = &chars[1];
                if symbols.contains(&second.as_str()) && second != "(" {
                    return Ok(FixType::PreFıx);
                }
            }

            if chars.len() > 2 {
                let third = &chars[2];
                if !symbols.contains(&third.as_str()) {
                    return Ok(FixType::PreFıx);
                }
            } else if chars.len() == 2 {
                return Ok(FixType::PreFıx);
            }
        }
    }

    Ok(FixType::Infıx)
}




pub(crate)  fn split_str(str: &str, split_symbols: &[&str], discard_symbols: &[char]) -> Vec<String> {
    let mut tokens: Vec<String> = Vec::new();
    let mut current_token = String::new();
    
    let chars: Vec<char> = str.chars().collect();
    let mut i = 0;

    'main: while i < chars.len() {
        let mut matched_symbol: Option<&str> = None;

        for &sym in split_symbols {
            if i + sym.len() <= chars.len() {
                let slice = &chars[i..i+sym.len()];
                let slice_str: String = slice.iter().collect();
                
                if slice_str == sym {
                    match matched_symbol {
                        Some(current_match) => {
                            if sym.len() > current_match.len() {
                                matched_symbol = Some(sym);
                            }
                        },
                        None => matched_symbol = Some(sym),
                    }
                }
            }
        }

        if let Some(symbol) = matched_symbol {
            if symbol == "-" && tokens.is_empty() && current_token.is_empty() {
                current_token.push('-');
                i += 1; // "-" karakterini geç
                continue 'main;
            }
            if !current_token.is_empty() {
                tokens.push(current_token.clone());
                current_token.clear();
            }

            let should_discard = if symbol.chars().count() == 1 {
                let c = symbol.chars().next().unwrap();
                discard_symbols.contains(&c)
            } else {
                false 
            };

            if !should_discard {
                tokens.push(symbol.to_string());
            }

            i += symbol.chars().count();
            continue 'main;
        }

        let current_char = chars[i];
        if discard_symbols.contains(&current_char) {
            if !current_token.is_empty() {
                tokens.push(current_token.clone());
                current_token.clear();
            }
            i += 1;
            continue 'main;
        }

        current_token.push(current_char);
        i += 1;
    }

    if !current_token.is_empty() {
        tokens.push(current_token);
    }

    tokens
}

