pub struct ConfirmState{
    pub show: bool,
    pub label: String,
    on_cancel: Option<Box<dyn FnOnce()>>,
    on_accept: Option<Box<dyn FnOnce()>>,
}


impl ConfirmState {
    pub fn new() -> Self {
        Self {
            show: false,
            label: "".into(),
            on_cancel: None,
            on_accept: None,
        }
    }

    pub fn open(
        &mut self,
        label: String, 
        on_accept: impl FnOnce() + 'static,
        on_cancel: impl FnOnce() + 'static,
    ) {
        self.show= true;
        self.label= label;
        self.on_cancel= Some(Box::new(on_cancel));
        self.on_accept= Some(Box::new(on_accept));
    }

    pub fn cancel(&mut self) {
        if let Some(f) = self.on_cancel.take() {
            f();
        }
        self.show = false;
    }

    pub fn accept(&mut self) {
        if let Some(f) = self.on_accept.take() {
            f();
        }
        self.show = false;
    }
    

}