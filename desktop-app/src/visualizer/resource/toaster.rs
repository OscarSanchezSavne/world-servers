use bevy::prelude::*;

#[derive(Resource)]
pub struct Toaster {
    pub messages: Vec<ToasterMessage>,
}

#[derive(Clone)]
pub struct ToasterMessage {
    pub text: String,
    pub elapsed: f32,
}

impl Toaster {
    pub fn default() -> Self {
        Self{
            messages: Vec::new(),
        }
    }

    pub fn add(&mut self, message: String)
    {
        self.messages.push(ToasterMessage{
            text: message,
            elapsed: 0.0,
        });
    }

    pub fn get_messages(&mut self, delta_secs: f32)-> Vec<ToasterMessage>
    {
        for message in self.messages.iter_mut() {
            message.elapsed+= delta_secs;
        }

        self.messages = std::mem::take(&mut self.messages)
            .into_iter()
            .filter(|message| message.elapsed <= 3.0)
            .collect();

        self.messages.clone()
    }


}
