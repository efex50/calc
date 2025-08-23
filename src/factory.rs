

pub struct Factory{
    pub state:  FactoryState,    
    pub str:    String
}

pub enum FactoryState{
    Null,
    Stored,
}

impl Factory {
    pub fn new()->Self{
       Self { 
            state: FactoryState::Null,
            str:String::new(),
       } 
    }
    pub fn insert(&mut self,str:String){
        self.str = format!("{}{}",&self.str,str)
        

    }
}
