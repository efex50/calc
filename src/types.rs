
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


#[derive(Debug,Clone, Copy)]
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
}




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
