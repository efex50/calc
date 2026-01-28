use std::ops::{Add, Div, Mul, Sub};


#[derive(Debug,Clone)]
pub enum TreeTokens{
    Variable(String),
    Number(Number),
    /// + 
    Plus,
    /// - 
    Sub,
    /// * 
    Mul,
    /// / 
    Div,
    BracO,
    BracC,


}


#[derive(Debug,Clone, Copy,)]
pub enum Number {
    Float(f64),
    Number(i128),
}

impl Add for Number {
    type Output = Number;
    fn add(self, other: Self) -> Self::Output {
        match (self, other) {
            (Number::Float(a), Number::Float(b)) => Number::Float(a + b),
            (Number::Float(a), Number::Number(b)) => Number::Float(a + b as f64),
            (Number::Number(a), Number::Float(b)) => Number::Float(a as f64 +b),
            (Number::Number(a), Number::Number(b)) => Number::Number(a+b),
        }
    }
}

impl From<i128> for Number {
    fn from(value: i128) -> Self {
        Self::Number(value)
    }
}


impl From<f64> for Number {
    fn from(value: f64) -> Self {
        Self::Float(value)
    }
}

impl Sub for Number {
    type Output = Number;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self,rhs) {
            (Number::Float(a), Number::Float(b)) => {
                return Number::Float(a-b);
            },
            (Number::Float(b), Number::Number(a)) |
            (Number::Number(a), Number::Float(b)) => {
                return Number::Float(a as f64-b);
            },
            (Number::Number(a), Number::Number(b)) => {
                return Number::Number(a-b);
            },
        }       

    }
}
impl Mul for Number {
    type Output = Number;
    fn mul(self, rhs: Self) -> Self::Output {
        match (self,rhs) {
            (Number::Float(a), Number::Float(b)) => {
                return Number::Float(a*b);
            },
            (Number::Float(b), Number::Number(a)) |
            (Number::Number(a), Number::Float(b)) => {
                return Number::Float(a as f64*b);
            },
            (Number::Number(a), Number::Number(b)) => {
                return Number::Number(a*b);
            },
        }       

    }
}
impl Div for Number {
    type Output = Number;

    fn div(self, rhs: Self) -> Self::Output {
        match (self,rhs) {
            (Number::Float(a), Number::Float(b)) => {
                return Number::Float(a/b);
            },
            (Number::Float(b), Number::Number(a)) |
            (Number::Number(a), Number::Float(b)) => {
                return Number::Float(a as f64/b);
            },
            (Number::Number(a), Number::Number(b)) => {
                return Number::Number(a/b);
            },
        }       

    }
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
}



#[derive(Debug,Clone, Copy)]
pub enum Symbols {
    Plus,
    Sub,
    Mul,
    Div,
    Brac,
}


impl TryFrom<TreeTokens> for Symbols {
    type Error = ();
    fn try_from(value: TreeTokens) -> Result<Self,()> {
        match value {
            TreeTokens::Sub => Ok(Self::Sub),
            TreeTokens::Mul => Ok(Self::Mul),
            TreeTokens::Div => Ok(Self::Div),
            TreeTokens::Plus => Ok(Self::Plus),
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
    Sub(Box<TreeType>,Box<TreeType>),
    Mul(Box<TreeType>,Box<TreeType>),
    Div(Box<TreeType>,Box<TreeType>),
    Brac(Box<TreeType>),
}

#[derive(Debug)]
pub enum Thing{
    Variable(String),
    Number(Number),

}

