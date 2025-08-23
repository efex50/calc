use std::collections::HashMap;

use crate::{types::{Number, Symbols, TreeTokens}, util::as_i128};




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



#[derive(Debug,Clone)]
pub enum TreeType{
    //leafs
    Variable(String),
    Number(Number),
    //branches
    Plus(Box<TreeType>,Box<TreeType>),
    Min(Box<TreeType>,Box<TreeType>),
    Mul(Box<TreeType>,Box<TreeType>),
    Sub(Box<TreeType>,Box<TreeType>),
    Exponent(Box<TreeType>,Box<TreeType>),
    Brac(Box<TreeType>),
}
impl TreeType{
    fn try_leaf(a:&str) -> TreeType{
        
        if let Ok(i) = as_i128(a){
            return TreeType::Number(Number::Number(i));
        }
        // float check lazım
        else if let Ok(f) = a.parse::<f64>(){
            return TreeType::Number(Number::Float(f));
        }

        else {
            return TreeType::Variable(a.to_string());
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
    pub variables : HashMap<String,Number>,
}

impl Tree {

    pub fn new() -> Self{
        Self { inner: None, variables: HashMap::new() }
    }


    fn check_fix_type(str:&String)-> Result<FixType,()>{

        
        if str.is_empty(){
            return Err(());
        }
        
        let symbols = ["+","-","*","/","**","//",","];
        
        let chars:Vec<String> = split_str(&str, &symbols,&[","]);
        println!("{:?}",chars);
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
            Self::parse_str(str, tip);
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
    pub fn parse_str(str:String,fixtype:FixType){
        let symbols = ["+","-","*","/","**","//",","];
        let chars:Vec<String> = split_str(&str, &symbols,&[","]);
        let tokens = Self::tokenize(chars);

        match fixtype {
            FixType::Infıx => Self::parse_infix(tokens),
            FixType::PostFıx => Self::parse_postfix(tokens),
            FixType::PreFıx => {Self::parse_prefix(&tokens,0);},
        };
    }

    fn parse_prefix(tokens:&Vec<TreeTokens>,start_from:usize) ->Result<TreeType,()>{
        if let Ok(symbol) =  Symbols::try_from(tokens[start_from].clone()) {
            
            match symbol {
                Symbols::Min => {
                    let left = TreeType::try_leaf(tokens[start_from+1]);

                    

                    
                },
                Symbols::Mul => {},
                Symbols::Sub => {},
                Symbols::Exp => {},
                Symbols::Plus => {},
                Symbols::Brac => {}, 
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
        
        
        
        


        todo!()
    }
    fn parse_postfix(tokens:Vec<TreeTokens>){
        
    }
    fn parse_infix(tokens:Vec<TreeTokens>){

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
            tokens.push(now);
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
                            if !tutan.is_empty(){
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
        let expected = vec!["a", "+", "v", "+", "34", "-", "ht", "-", "adfa", "*/*+", "j"];
        let expected: Vec<String> = expected.iter().map(|s| s.to_string()).collect();

        let res = split_str(&str.to_string(), &["+","-","*","/","*/*+"],&[]);
        assert_eq!(expected,res);
    }
}
