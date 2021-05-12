
#[derive(Debug, Clone)]
pub enum Event {
    Click(ClickPar),
    Drag(DragPar)
}

#[derive(Debug, Clone)]
pub struct ClickPar {
    action_id:String,
}



impl ClickPar {
    pub fn new(action_id:&String) -> Self {
        Self{action_id:action_id.to_owned()}
    }

    pub fn action_id(&self) -> &str {
        &self.action_id
    }
}


#[derive(Debug, Clone)]
pub struct DragPar {
    action_id:String,
    value:f32,
    in_progress:bool,
    cancelled:bool,
}

impl DragPar {

    pub fn in_progress(action_id:String,value:f32) ->Self {
        Self{action_id:action_id.to_owned(),value, in_progress:true,cancelled:false}
    }

    pub fn cancelled(action_id:String,value:f32) ->Self {
        Self{action_id:action_id.to_owned(),value, in_progress:false,cancelled:true}
    }

    pub fn done(action_id:String,value:f32) ->Self {
        Self{action_id:action_id.to_owned(),value, in_progress:false,cancelled:false}
    }

    pub fn is_in_progress(&self) -> bool {
        self.in_progress
    }
    pub fn is_cancelled(&self) -> bool {
        self.cancelled
    }

    pub fn action_id(&self) -> &str {
        &self.action_id
    }
    pub fn value(&self) -> f32 {
        self.value
    }
}