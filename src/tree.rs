use std::{collections::HashMap, thread::yield_now};

use crate::{types::{ Number, Symbols, Thing, TreeTokens, TreeType}, util::as_i128};




macro_rules! add_fix {
    (pre $fixtype:ident $str:ident $t:expr ) => {
        
        match $fixtype {
            FixType::Infıx => (),
            FixType::PostFıx => (),
            FixType::PreFıx => $str.push($t),
        }
    };
    (post $fixtype:ident $str:ident $t:expr) => {
        match $fixtype {
            FixType::Infıx => (),
            FixType::PostFıx => $str.push($t),
            FixType::PreFıx => (),
        }

    };
    (prepo $fixtype:ident $str:ident $t:expr) => {
        match $fixtype {
            FixType::Infıx => (),
            FixType::PostFıx => $str.push($t),
            FixType::PreFıx => $str.push($t),
        }

    };
    
    (infix $fixtype:ident $str:ident $t:expr) => {
        match $fixtype {
            FixType::Infıx => $str.push($t),
            FixType::PostFıx => (),
            FixType::PreFıx => (),
        }

    }

}





#[derive(Debug)]
pub enum FixType{
    Infıx,
    PostFıx,
    PreFıx,
}

#[derive(Debug)]
pub struct Tree{
    pub inner : Option<Box::<TreeType>>,
    pub variables : HashMap<String,Option<Number>>,
}

// parsing and printing
impl Tree {

    pub fn new() -> Self{
        Self { inner: None, variables: HashMap::new() }
    }


    fn check_fix_type(str:&String)-> Result<FixType,()>{

        
        if str.is_empty(){
            return Err(());
        }
        
        let symbols = ["+","-","*","/","**","//",","];
        
        let chars:Vec<String> = split_str(&str, &symbols,&[',',' ']);
        for x in symbols{
            if x == chars[0]{
                return Ok(FixType::PreFıx) ;
            }
            if x == chars[chars.len()-1]{
                return Ok(FixType::PostFıx);
            }
        };
        for x in chars{
            if symbols.contains(&x.as_str()){
                return Ok(FixType::Infıx);
            };
        }

        Err(())
    }


    /// valid strings are
    /// 
    /// 1+23+4242(a+b)
    /// 
    /// +23+54,23
    pub fn parse_auto(&mut self,str:String) -> Result<(),()> {
        let tip = Self::check_fix_type(&str);
        if let Ok(tip)=tip{
            Self::parse_str(self,str, tip)?;
        }else {
           return Err(()) 
        };
        Ok(())
    }

    /// valid strings are
    /// 
    /// 1+23+4242(a+b)
    /// 
    /// +23+54,23
    pub fn parse_str(&mut self,str:String,fixtype:FixType) -> Result<(),()>{
        let symbols = ["+","-","*","/","**","//",",","(",")"];
        let chars:Vec<String> = split_str(&str, &symbols,&[',',' ']);
        println!("{:?}",chars);
        let tokens = Self::tokenize(chars);
        //println!("{:?}",tokens);
        let a = match fixtype {
            FixType::Infıx => Self::parse_infix_main(&tokens,0),
            FixType::PostFıx => Self::parse_postfix(&tokens,&mut (tokens.len()-1)),
            FixType::PreFıx => Self::parse_prefix(&tokens,&mut 0),
        };
        if let Ok(a) = a {
            self.inner= Some(Box::new(a));
            Ok(())
        }else {
            Err(())
        }
    }

    fn maketree(symbol:Symbols,left:Box<TreeType>,right:Box<TreeType>) -> TreeType{
        match symbol {
            Symbols::Plus => TreeType::Plus(left, right),
            Symbols::Sub =>  TreeType::Sub(left, right),
            Symbols::Mul =>  TreeType::Mul(left, right),
            Symbols::Div =>  TreeType::Sub(left, right),
            Symbols::Brac => todo!(),
        }
    }
    fn parse_prefix(tokens: &Vec<TreeTokens>, index: &mut usize) -> Result<TreeType, ()> {
        
        if *index >= tokens.len() {
            return Err(());
        }

        let tok = &tokens[*index];
        *index += 1;

        match tok {
            TreeTokens::BracC => {
                Self::parse_prefix(tokens, index)
            },
            
            TreeTokens::Number(n) => Ok(TreeType::Number(*n)),
            TreeTokens::Variable(v) => Ok(TreeType::Variable(v.clone())),
            
            _ => {
                if let Ok(symbol) = Symbols::try_from(tok.clone()) {
                    match symbol {
                        Symbols::Mul | 
                        Symbols::Sub | 
                        Symbols::Plus | 
                        Symbols::Div => {
                            let left = Self::parse_prefix(tokens, index)?;
                            
                            let right = Self::parse_prefix(tokens, index)?;
                            
                            Ok(Self::maketree(symbol, Box::new(left), Box::new(right)))
                        },
                        
                        Symbols::Brac => {
                            let inner = Self::parse_prefix(tokens, index)?;
                            
                                if *index < tokens.len() {
                                if let TreeTokens::BracC = tokens[*index] {
                                    *index += 1; 
                                }
                            }
                            
                            Ok(TreeType::Brac(Box::new(inner)))
                        }
                    }
                } else {
                    Err(())
                }
            }
        }
    }
 fn parse_postfix(tokens: &Vec<TreeTokens>, index: &mut usize) -> Result<TreeType, ()> {
        let tok = &tokens[*index];
        let can_go_back = *index > 0;
        if can_go_back {
            *index -= 1;
        }

        match tok {
            TreeTokens::BracC => {
                let inner = Self::parse_postfix(tokens, index)?;

                if !can_go_back && *index == 0 { 
                } 
                
                if let TreeTokens::BracO = tokens[*index] {
                    if *index > 0 {
                        *index -= 1;
                    }
                } else {
                    return Err(());
                }

                Ok(TreeType::Brac(Box::new(inner)))
            },

            TreeTokens::Number(n) => Ok(TreeType::Number(*n)),
            TreeTokens::Variable(v) => Ok(TreeType::Variable(v.clone())),
            _ => {
                if let Ok(symbol) = Symbols::try_from(tok.clone()) {
                    match symbol {
                        Symbols::Mul | 
                        Symbols::Sub | // Senin kodunda Bölme (/) Sub olarak geçiyor olabilir
                        Symbols::Plus | 
                        Symbols::Div => {
                            // DİKKAT: Tersten okuduğumuz için önce SAĞ tarafı (Right) parse ediyoruz.
                            if !can_go_back { return Err(()); } // Operand yoksa hata
                            let right = Self::parse_postfix(tokens, index)?;

                            let left = Self::parse_postfix(tokens, index)?;

                            Ok(Self::maketree(symbol, Box::new(left), Box::new(right)))
                        },
                        _ => Err(())
                    }
                } else {
                    Err(())
                }
            }
        }
    }   
    fn parse_infix_main(tokens:&Vec<TreeTokens>,index:usize)->Result<TreeType,()>{
        let res = Self::parse_infix(tokens, &mut index.clone(),0);
        res
    }
    // pratt parser
    fn parse_infix<'a>(tokens:&Vec<TreeTokens>,index:&mut usize,min_bp:u8) -> Result<TreeType,()>{
        if *index >= tokens.len() {
            return Err(());
        }
        // sol tarafı parse'le ve ilerle
        let current_token = &tokens[*index];
        *index += 1; 

        let mut lhs = match current_token {
            TreeTokens::Number(n) => TreeType::Number(*n),
            TreeTokens::Variable(v) => TreeType::Variable(v.clone()),
            TreeTokens::BracO => {
                // parantez açılırsa bp sıfırla
                let inner = Self::parse_infix(tokens, index, 0)?;
                
                // Kapanış parantezini kontrol et ve tüket
                if *index < tokens.len() {
                    if let TreeTokens::BracC = tokens[*index] {
                        *index += 1;
                    } else {
                         return Err(());
                    }
                } else {
                    return Err(());
                }
                
                TreeType::Brac(Box::new(inner))
            },
            _ => return Err(()), 
        };
        loop {
            if *index >= tokens.len() {
                break;
            }

            let op_token = &tokens[*index];

            let (l_bp, r_bp) = match get_binding_power(op_token) {
                Some(bp) => bp,
                None => break, // Operatör değilse döngüden çık.
            };

            // eğer sol güç düşükse bu üstteki recursive çağrıya aits
            if l_bp < min_bp {
                break;
            }

            // Operatörü tüket ve kopyasını al
            let op = op_token.clone();
            *index += 1;

            // Sağ tarafı (RHS) parse et.
            // Yeni minimum güç olarak operatörün SAĞ gücünü (r_bp) veriyoruz.
            let rhs = Self::parse_infix(tokens, index, r_bp)?;

            // LHS ve RHS'yi birleştir
            lhs = Self::maketree(
                Symbols::try_from(op).map_err(|_| ())?, 
                Box::new(lhs), 
                Box::new(rhs)
            );
        };
        Ok(lhs)
    }
    
    fn tokenize(str:Vec<String>) -> Vec<TreeTokens>{
        let mut tokens:Vec<TreeTokens> = Vec::new();
        for x in str{
            match x.as_str() {
                
                "+" => tokens.push(TreeTokens::Plus),
                "-" => tokens.push(TreeTokens::Sub),
                "*" => tokens.push(TreeTokens::Mul),
                "/" => tokens.push(TreeTokens::Div),
                "(" => tokens.push(TreeTokens::BracO),
                ")" => tokens.push(TreeTokens::BracC),
                _ => {
                    
                    if let Ok(a) = as_i128(&x){
                        tokens.push(TreeTokens::Number(Number::Number(a)));
                    }else if let Ok(b) = x.parse() {
                        tokens.push(TreeTokens::Number(Number::Float(b)));   
                    }
                    else {
                        tokens.push(TreeTokens::Variable(x));
                    }
                }
            }
        }


        tokens
    }
    
    pub fn prefixprint(&self) -> Result<String,()>{
        let str: String ;

        if let Some(inner) = &self.inner{
            str = Self::print_inner(&*inner,&FixType::PreFıx)
        }else {
            return Err(());
        }
        Ok(str)
    }
    pub fn postfixprint(&self) -> Result<String,()>{
        let str: String ;

        if let Some(inner) = &self.inner{
            str = Self::print_inner(&*inner,&FixType::PostFıx)
        }else {
            return Err(());
        }
        Ok(str)
    }
    pub fn infixprint(&self) -> Result<String,()>{
        let str: String ;

        if let Some(inner) = &self.inner{
            str = Self::print_inner(&*inner,&FixType::Infıx)
        }else {
            return Err(());
        }
        Ok(str)
    }

    fn print_inner(tree:&TreeType,fixtype:&FixType) -> String{
        let mut str = String::new();

        match tree {
            TreeType::Variable(name) => { 
                        str = format!("{}{}",str,name)
                    },
            TreeType::Number(number) => {
                        str = format!("{str}{}",number.as_str())
                    },
            TreeType::Plus(tree_type, tree_type1) => {
                        add_fix!(pre fixtype str '+');
                        let l = Self::print_inner(&*tree_type,fixtype);
                        let r = Self::print_inner(&*tree_type1,fixtype);
                        str.push_str(&l);
                        add_fix!(infix fixtype str '+');
                        add_fix!(prepo fixtype str ',');
                        str.push_str(&r);
                        add_fix!(post fixtype str '+');

                    },
            TreeType::Div(tree_type, tree_type1) => {
                        add_fix!(pre fixtype str '-');
                        let l = Self::print_inner(&*tree_type,fixtype);
                        let r = Self::print_inner(&*tree_type1,fixtype);
                        str.push_str(&l);
                        add_fix!(infix fixtype str '-');
                        add_fix!(prepo fixtype str ',');
                        str.push_str(&r);
                        add_fix!(post fixtype str '-');

                    },
            TreeType::Mul(tree_type, tree_type1) => {
                        add_fix!(pre fixtype str '*');
                        let l = Self::print_inner(&*tree_type,fixtype);
                        let r = Self::print_inner(&*tree_type1,fixtype);
                        str.push_str(&l);
                        add_fix!(infix fixtype str '*');
                        add_fix!(prepo fixtype str ',');
                        str.push_str(&r);
                        add_fix!(post fixtype str '*');

                    },
            TreeType::Sub(tree_type, tree_type1) => {
                        add_fix!(pre fixtype str '/');
                        let l = Self::print_inner(&*tree_type,fixtype);
                        let r = Self::print_inner(&*tree_type1,fixtype);
                        str.push_str(&l);
                        add_fix!(infix fixtype str '/');
                        add_fix!(prepo fixtype str ',');
                        str.push_str(&r);
                        add_fix!(post fixtype str '/');
                    },
            TreeType::Brac(tree_type) => {
                        // /*
                        //str.push('(');
                        let l = Self::print_inner(&*tree_type,fixtype);
                        str.push_str(&l);
                        //str.push(')');
                        // */
                    },
        }


        return str;
    }


}

impl Tree {
    pub fn simplify(&mut self) {
        // self.inner'ın sahipliğini alıyoruz (take), işliyoruz ve geri koyuyoruz.
        if let Some(tree) = self.inner.take() {
            match Self::inner_simplify(tree) {
                // Eğer sonuç bir sayıya indirgendiyse (örn: 3+5 -> 8), ağaç artık sadece o sayıdır.
                Ok(number) => self.inner = Some(Box::new(TreeType::Number(number))),
                // Eğer tam indirgenemediyse (örn: x+5), dönen sadeleşmiş ağacı koyuyoruz.
                Err(node) => self.inner = Some(node),
            }
        }
    }

    // Dönüş tipi: Ok(Number) -> Tamamen sadeleşti, Err(Box<TreeType>) -> Ağaç olarak kaldı
    fn inner_simplify(tree: Box<TreeType>) -> Result<Number, Box<TreeType>> {
        match *tree {
            // Değişken sadeleşemez, olduğu gibi ağaç (Err) olarak döner.
            TreeType::Variable(_) => Err(tree),
            
            // Sayı zaten sadedir, değer (Ok) olarak döner.
            TreeType::Number(number) => Ok(number),
            
            // Parantez içi sadeleşmeye çalışılır
            TreeType::Brac(inner) => Self::inner_simplify(inner),

            // --- İŞLEMLER ---
            
            TreeType::Plus(left, right) => {
                let l = Self::inner_simplify(left);
                let r = Self::inner_simplify(right);
                
                match (l, r) {
                    // İki taraf da sayı olduysa topla: 3 + 5 -> 8
                    (Ok(n1), Ok(n2)) => Ok(n1 + n2),
                    
                    // En az biri sayı değilse ağacı yeniden kur: x + (2+3) -> x + 5
                    (l_res, r_res) => {
                        let l_node = l_res.unwrap_or_else(|node| Box::new(TreeType::Number(node))); // Ok ise Number node yap, Err ise node'u al
                        let r_node = l_res.unwrap_or_else(|node| Box::new(TreeType::Number(node))); // *Hata düzeltmesi aşağıda
                        
                        // Yardımcı fonksiyon kullanarak temizleyelim, yukarıdaki unwrap_or logic'i karışık olabilir.
                        // Aşağıdaki `reconstruct` mantığı daha temizdir.
                        Err(Box::new(TreeType::Plus(
                            Self::result_to_node(l_res), 
                            Self::result_to_node(r_res)
                        )))
                    }
                }
            },
            
            TreeType::Sub(left, right) => { 
                let l = Self::inner_simplify(left);
                let r = Self::inner_simplify(right);
                match (l, r) {
                    (Ok(n1), Ok(n2)) => Ok(n1 -n2),
                    (l_res, r_res) => Err(Box::new(TreeType::Sub(Self::result_to_node(l_res), Self::result_to_node(r_res)))),
                }
            },
            
            TreeType::Mul(left, right) => { 
                let l = Self::inner_simplify(left);
                let r = Self::inner_simplify(right);
                match (l, r) {
                    (Ok(n1), Ok(n2)) => Ok(n1*n2),
                    (l_res, r_res) => Err(Box::new(TreeType::Mul(Self::result_to_node(l_res), Self::result_to_node(r_res)))),
                }
            },
            
            TreeType::Div(left, right) => { 
                let l = Self::inner_simplify(left);
                let r = Self::inner_simplify(right);
                match (l, r) {
                    (Ok(n1), Ok(n2)) => Ok(n1 / n2),
                    (l_res, r_res) => Err(Box::new(TreeType::Div(Self::result_to_node(l_res), Self::result_to_node(r_res)))),
                }
            },
            
        }
    }

    // Helper: Result'ı tekrar Tree Node'una çevirir
    fn result_to_node(res: Result<Number, Box<TreeType>>) -> Box<TreeType> {
        match res {
            Ok(num) => Box::new(TreeType::Number(num)),
            Err(node) => node,
        }
    }
}

fn get_binding_power(op: &TreeTokens) -> Option<(u8, u8)> {
    match op {
        TreeTokens::Plus | TreeTokens::Sub => Some((1, 2)),        
        TreeTokens::Mul | TreeTokens::Div => Some((3, 4)),
        _ => None,
    }
}

pub fn split_str(str: &str, split_symbols: &[&str], discard_symbols: &[char]) -> Vec<String> {
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






#[cfg(test)]
pub mod test{
    use crate::tree::split_str;

    #[test]
    fn split_test(){
        let str = "a+v+34-ht-adfa*/*+j";
        let expected = vec!["a", "+", "v", "+", "34", "-", "ht", "-", "adfa", "*/*","+", "j"];
        let expected: Vec<String> = expected.iter().map(|s| s.to_string()).collect();
        let res = split_str(&str.to_string(), &["+","-","*","/","*/*"],&[]);
        assert_eq!(expected,res);
    }
    #[test]
    fn sdadt(){
        let str = "+5 5";
        let res = split_str(str, &["+","-","/","(",")","*"], &[',',' ']);
        println!("{:?}",res);
    }
}
