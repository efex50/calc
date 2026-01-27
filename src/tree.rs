use std::collections::HashMap;

use crate::{types::{InfixTree,  Number, Symbols, Thing, TreeTokens, TreeType}, util::as_i128};




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
        
        let chars:Vec<String> = split_str(&str, &symbols,&[",",""]);
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
        let chars:Vec<String> = split_str(&str, &symbols,&[","]);
        println!("{:?}",chars);
        let tokens = Self::tokenize(chars);
        //println!("{:?}",tokens);
        let a = match fixtype {
            FixType::Infıx => Self::parse_infix_main(&tokens,0),
            FixType::PostFıx => Self::parse_postfix(&tokens,tokens.len()-1),
            FixType::PreFıx => Self::parse_prefix(&tokens,0),
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
            Symbols::Min =>  TreeType::Min(left, right),
            Symbols::Mul =>  TreeType::Mul(left, right),
            Symbols::Sub =>  TreeType::Sub(left, right),
            Symbols::Exp =>  TreeType::Exponent(left, right),
            Symbols::Brac => todo!(),
        }
    }

    fn parse_prefix(tokens:&Vec<TreeTokens>,start_from:usize) ->Result<TreeType,()>{
        let tok =  tokens.get(start_from).ok_or_else(||())?;
        dbg!(&tok);
        match tok {
            TreeTokens::BracC => {
                dbg!("adflaföailföa");
                return Self::parse_prefix(tokens, start_from+1);
            },
            _=>()
        };

        if let Ok(symbol) =  Symbols::try_from(tok.clone()) {
            
            match symbol {
                Symbols::Mul |
                Symbols::Sub |
                Symbols::Exp |
                Symbols::Plus |
                Symbols::Min => {
                    let left = TreeType::try_leaf(&tokens[start_from+1]);
                    let left = if let Ok(l) = left{
                            l
                    }else{
                        match Self::parse_prefix(tokens, start_from+1){
                            Ok(o) => o,
                            Err(_) => return Err(()),
                        }
                    };
                    
                    
                    let right = TreeType::try_leaf(&tokens[start_from+2]);
                    let right = if let Ok(l) = right{
                            l
                    }else{
                        match Self::parse_prefix(tokens, start_from+2){
                            Ok(o) => o,
                            Err(_) => return Err(()),
                        }
                    };
                    let t = Self::maketree(symbol,Box::new(left), Box::new(right));
                    return Ok(t);                    

                    
                },
                Symbols::Brac => {
                    let inner = Self::parse_prefix(tokens, start_from+1)?;
                    let t = TreeType::Brac(Box::new(inner));
                    return  Ok(t);
                }, 
            };
        }else {
            match &tok {
                TreeTokens::Number(a) => {
                    let leaf = TreeType::Number(a.clone());
                    return Ok(leaf);
                },
                TreeTokens::Variable(v) => {
                    let leaf = TreeType::Variable(v.clone());
                    return Ok(leaf);

                },
                _=> return Err(())
            }
        }
        
        
    }
    fn parse_postfix(tokens:&Vec<TreeTokens>,start_from:usize)->Result<TreeType,()>{
            if let Ok(symbol) =  Symbols::try_from(tokens[start_from].clone()) {
            
            match symbol {
                Symbols::Mul |
                Symbols::Sub |
                Symbols::Exp |
                Symbols::Plus |
                Symbols::Min => {
                    let left = TreeType::try_leaf(&tokens[start_from-1]);
                    let left = if let Ok(l) = left{
                            l
                    }else{
                        match Self::parse_postfix(tokens, start_from-1){
                            Ok(o) => o,
                            Err(_) => return Err(()),
                        }
                    };
                    
                    
                    let right = TreeType::try_leaf(&tokens[start_from-2]);
                    let right = if let Ok(l) = right{
                            l
                    }else{
                        match Self::parse_postfix(tokens, start_from-2){
                            Ok(o) => o,
                            Err(_) => return Err(()),
                        }
                    };
                    let t = Self::maketree(symbol,Box::new(left), Box::new(right));
                    return Ok(t);                    

                    
                },
                Symbols::Brac => {
                    let mut opens = 0;
                    let mut ctr = start_from -1;
                    loop {
                        let next = &tokens[ctr];
                        dbg!(&next);
                        match next {
                            TreeTokens::BracO => opens +=1,
                            TreeTokens::BracC => {
                                if opens == 0{
                                    if let Ok(t) = Self::parse_postfix(tokens, ctr+1){
                                        let brac = TreeType::Brac(Box::new(t));
                                        return Ok(brac);
                                    }

                                }else {
                                    opens -= 1;
                                }
                            },
                            _=> (),
                        };
                        ctr -=1;
                        if ctr >= tokens.len(){
                            return Err(());
                        }
                    }
                }, 
            };
        }else {
            match &tokens[0] {
                TreeTokens::Number(a) => {
                    let leaf = TreeType::Number(a.clone());
                    return Ok(leaf);
                },
                TreeTokens::Variable(v) => {
                    let leaf = TreeType::Variable(v.clone());
                    return Ok(leaf);

                },
                _=> return Err(())
            }
        }
        
        
        
        


    }
    fn parse_infix_main(tokens:&Vec<TreeTokens>,ctr:usize)->Result<TreeType,()>{
        let iter  = tokens.iter().peekable();
        Self::parse_infix(iter, ctr);
        todo!()
    }
    fn parse_infix<'a>(tokens:impl Iterator<Item = &'a TreeTokens>,ctr:usize) -> Result<InfixTree,()>{
        // rewrite with iterator        
        
        todo!()
    }
    
    fn tokenize(str:Vec<String>) -> Vec<TreeTokens>{
        let mut tokens:Vec<TreeTokens> = Vec::new();
        for x in str{
            match x.as_str() {
                
                "+" => tokens.push(TreeTokens::Plus),
                "-" => tokens.push(TreeTokens::Min),
                "*" => tokens.push(TreeTokens::Mul),
                "/" => tokens.push(TreeTokens::Sub),
                "**" => tokens.push(TreeTokens::Exponent),
                "(" => tokens.push(TreeTokens::BracO),
                ")" => tokens.push(TreeTokens::BracC),
                _ => {
                    let a = as_i128(&x);
                    let b:Result<f64, _> = x.parse();
                    
                    if let Ok(a) = a{
                        tokens.push(TreeTokens::Number(Number::Number(a)));
                    }else if let Ok(b) = b {
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
            TreeType::Min(tree_type, tree_type1) => {
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
                        str.push('(');
                        let l = Self::print_inner(&*tree_type,fixtype);
                        str.push_str(&l);
                        str.push(')');
                    },
            TreeType::Exponent(tree_type, tree_type1) => {
                        add_fix!(pre fixtype str '*');
                        add_fix!(pre fixtype str '*');
                        let l = Self::print_inner(&*tree_type,fixtype);
                        let r = Self::print_inner(&*tree_type1,fixtype);
                        str.push_str(&l);
                        add_fix!(infix fixtype str '*');
                        add_fix!(infix fixtype str '*');
                        add_fix!(prepo fixtype str ',');
                        str.push_str(&r);
                        add_fix!(post fixtype str '*');
                        add_fix!(post fixtype str '*');

                    },
        }


        return str;
    }


}

// arithmetics
impl Tree {
    pub fn simplify(&mut self){

    }

    fn inner_simplify(tree:Box<TreeType>) -> Result<Number,Box<TreeType>>{
        match *tree {
            TreeType::Variable(_) => return Err(tree),
            TreeType::Number(number) => return Ok(number),
            TreeType::Plus(tree_type, tree_type1) =>{
                let l = Self::inner_simplify(tree_type);
                let r = Self::inner_simplify(tree_type1);
                if let (Ok(l),Ok(r)) = (l,r){
                    return Ok(Number::add(&l, &r));
                }

            }
            TreeType::Min(tree_type, tree_type1) =>{
                let l = Self::inner_simplify(tree_type);
                let r = Self::inner_simplify(tree_type1);
                if let (Ok(l),Ok(r)) = (l,r){
                    return Ok(Number::sub(&l, &r));
                }

            }
            TreeType::Mul(tree_type, tree_type1) =>{
                let l = Self::inner_simplify(tree_type);
                let r = Self::inner_simplify(tree_type1);
                if let (Ok(l),Ok(r)) = (l,r){
                    todo!();
                }

            }
            TreeType::Sub(tree_type, tree_type1) =>{
                let l = Self::inner_simplify(tree_type);
                let r = Self::inner_simplify(tree_type1);
                if let (Ok(l),Ok(r)) = (l,r){
                    todo!();
                }

            }
            TreeType::Exponent(tree_type, tree_type1) => {
                let l = Self::inner_simplify(tree_type);
                let r = Self::inner_simplify(tree_type1);
                if let (Ok(l),Ok(r)) = (l,r){
                    todo!();
                }
            },
            
            TreeType::Brac(tree_type) => {
                return Self::inner_simplify(tree_type);
            },
        };

        todo!()
    }
}


fn get_binding_power(op: &TreeTokens) -> Option<(u8, u8)> {
    match op {
        // Düşük Öncelik: Toplama ve Çıkarma (+, -)
        // Sol ilişkili (1, 2) -> (1 + 2) + 3
        TreeTokens::Plus | TreeTokens::Min => Some((1, 2)),
        
        // Orta Öncelik: Çarpma ve Bölme (*, /) -> Kodunuzda '/' Sub olarak geçiyor
        // Sol ilişkili (3, 4) -> (3 * 4) / 5
        TreeTokens::Mul | TreeTokens::Sub => Some((3, 4)),
        
        // Yüksek Öncelik: Üs Alma (**)
        // Sağ ilişkili (6, 5) -> 2 ** 3 ** 4 = 2 ** (3 ** 4)
        TreeTokens::Exponent => Some((6, 5)),
        
        // Diğerleri operatör değil
        _ => None,
    }
}


pub fn split_str(str:&String,split_symbols:&[&str],discard_symbols:&[&str]) -> Vec<String>{
    
    let mut max_symbol_size = 0;
    for x in split_symbols{
        let len = x.len();
        if len > max_symbol_size{
            max_symbol_size = len;
        }
    }
    let mut tokens:Vec<String> = Vec::new();
    let mut now = String::new();
    let mut chars = str.split("").collect::<Vec<&str>>();
    chars.pop();
    let mut ctr = 1;

    'main:loop {
        if ctr >= chars.len(){
            if now.is_empty(){

            }else {   
                tokens.push(now);
            }
            break 'main;
        }
        let c = chars[ctr];
        if split_symbols.contains(&c){
            if !now.is_empty(){
                tokens.push(now.clone());
                now.clear();
            }
            if !discard_symbols.contains(&c){
                
                
                if max_symbol_size == 1{
                    tokens.push(c.to_string());
                }
                
                
                
                else {
                    let old = ctr;
                    let mut size:usize = 1;
                    let mut longchar = String::from(c);
                    let mut tutan = String::new();
                   
                    
                    loop {
                        
                        // eğer sembol limiti dolduysa 
                        if size == max_symbol_size{
                            // burada break at lo
                            // ve eklenen bişeye benziyosa
                            if !tutan.is_empty() && !discard_symbols.contains(&tutan.as_str()){
                                ctr += tutan.len();

                                tokens.push(tutan);
                                continue 'main;
                            }
                            ctr += 1;

                            tokens.push(c.to_string());
                            continue 'main;
                        }


                        let next = chars.get(old+size);
                        // eğer next varsa
                        if let Some(next) = next{
                            longchar.push_str(&next);
                            if split_symbols.contains(&longchar.as_str()){
                                tutan.push_str(&longchar);
                            }
                            
                        }                        
                        size += 1;
                    }
                }



            }
            ctr +=1;
            continue 'main;
        }
        now.push_str(c);
        
        
        ctr += 1;
        
    }

    return tokens;
    
    
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
}
