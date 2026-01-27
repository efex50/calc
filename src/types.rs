
#[derive(Debug,Clone)]
pub enum TreeTokens{
    Variable(String),
    Number(Number),
    Plus,
    Min,
    Mul,
    Sub,
    Exponent,
    BracO,
    BracC,


}


#[derive(Debug,Clone, Copy,)]
pub enum Number {
    Float(f64),
    Number(i128),
}
impl Number {
    pub fn as_str(&self) -> String{
        match self {
            Number::Float(a) =>{
                format!("{}",a)
            }
            Number::Number(a) => {
                format!("{}",a)
            },
        }
    }
    pub fn add(&self,other :&Number) -> Number{
        match (self,other) {
            (Number::Float(a), Number::Float(b)) => {
                return Number::Float(a+b);
            },
            (Number::Float(b), Number::Number(a)) |
            (Number::Number(a), Number::Float(b)) => {
                return Number::Float(*a as f64+b);
            },
            (Number::Number(a), Number::Number(b)) => {
                return Number::Number(a+b);
            },
        }       
    }
    pub fn sub(&self,other :&Number) -> Number{
        match (self,other) {
            (Number::Float(a), Number::Float(b)) => {
                return Number::Float(a-b);
            },
            (Number::Float(b), Number::Number(a)) |
            (Number::Number(a), Number::Float(b)) => {
                return Number::Float(*a as f64-b);
            },
            (Number::Number(a), Number::Number(b)) => {
                return Number::Number(a-b);
            },
        }       
    }
    pub fn mul(&self,other :&Number) -> Number{
        match (self,other) {
            (Number::Float(a), Number::Float(b)) => {
                return Number::Float(a*b);
            },
            (Number::Float(b), Number::Number(a)) |
            (Number::Number(a), Number::Float(b)) => {
                return Number::Float(*a as f64*b);
            },
            (Number::Number(a), Number::Number(b)) => {
                return Number::Number(a*b);
            },
        }       
    }
    pub fn divide(&self,other :&Number) -> Number{
        match (self,other) {
            (Number::Float(a), Number::Float(b)) => {
                return Number::Float(a/b);
            },
            (Number::Float(b), Number::Number(a)) |
            (Number::Number(a), Number::Float(b)) => {
                return Number::Float(*a as f64/b);
            },
            (Number::Number(a), Number::Number(b)) => {
                return Number::Number(a/b);
            },
        }       
    }
    pub fn pow(&self,other :&Number) -> Number{
        match (self,other) {
            (Number::Float(a), Number::Float(b)) => {
                return Number::Float(a**b);
            },
            (Number::Float(_b), Number::Number(_a)) |
            (Number::Number(_a), Number::Float(_b)) => {
                todo!("not yet implemented")
            },
            (Number::Number(a), Number::Number(b)) => {
                return Number::Number(a**b);
            },
        }       

    }
}



#[derive(Debug,Clone, Copy)]
pub enum Symbols {
    Plus,
    Min,
    Mul,
    Sub,
    Exp,
    Brac,
}


impl TryFrom<TreeTokens> for Symbols {
    type Error = ();
    fn try_from(value: TreeTokens) -> Result<Self,()> {
        match value {
            TreeTokens::Min => Ok(Self::Min),
            TreeTokens::Mul => Ok(Self::Mul),
            TreeTokens::Sub => Ok(Self::Sub),
            TreeTokens::Plus => Ok(Self::Plus),
            TreeTokens::Exponent => Ok(Self::Exp),
            TreeTokens::BracO => Ok(Self::Brac),
            _ => Err(())
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
    pub(crate) fn try_leaf(a:&TreeTokens) -> Result<TreeType,()>{
        
        match a {
            TreeTokens::Variable(a) => Ok(TreeType::Variable(a.to_string())),
            TreeTokens::Number(number) => Ok(TreeType::Number(*number)),
            _=> return Err(())
        }

    
    }
}

#[derive(Debug)]
pub enum Thing{
    Variable(String),
    Number(Number),

}

#[derive(Debug)]
pub enum InfixTree{
    //leafs
    Thing(Thing),
    //branches
    Op{
        sym:Symbols,
        left:Box<InfixTree>,
        right:Box<InfixTree>
    },
    // op half
    Oph{
        sym:Symbols,
        left:Box<InfixTree>
    },
}
/*impl InfixTree {

    pub fn to_symbol(&self) -> Option<Symbols>{
        match self {
            InfixTree::Variable(_) => None,
            InfixTree::Number(_number) => None,
            InfixTree::Plus(_infix_tree) => {Some(Symbols::Plus)},
            InfixTree::Min(_infix_tree) => {Some(Symbols::Min)},
            InfixTree::Mul(_infix_tree) => {Some(Symbols::Mul)},
            InfixTree::Sub(_infix_tree) => {Some(Symbols::Sub)},
            InfixTree::Exponent(_infix_tree) => {Some(Symbols::Exp)},
            InfixTree::Brac(_tree_type) => {Some(Symbols::Brac)},
        }
    }
    
    pub fn merge(&mut self,other:Self){
        match self {
            InfixTree::Variable(_) => todo!(),
            InfixTree::Number(number) => todo!(),
            InfixTree::Plus(infix_tree) => {
                let t = InfixTree::Plus(Box::new(other));
                *self = t;
            },
            InfixTree::Min(infix_tree) => {
                let t = InfixTree::Plus(Box::new(other));
                *self = t;
            },
            InfixTree::Mul(infix_tree) => {
                let t = InfixTree::Plus(Box::new(other));
                *self = t;
            },
            InfixTree::Sub(infix_tree) => {
                let t = InfixTree::Plus(Box::new(other));
                *self = t;
            },
            InfixTree::Exponent(infix_tree) => {
                let t = InfixTree::Plus(Box::new(other));
                *self = t;
            },
            InfixTree::Brac(infix_tree) => {
                let t = InfixTree::Plus(Box::new(other));
                *self = t;
            },
        }
    }
}

*/