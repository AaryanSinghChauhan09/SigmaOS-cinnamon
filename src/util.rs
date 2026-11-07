//! cinnamon model — Cinnamon desktop environment.

#[derive(Debug, Clone)]
pub struct Component {
    pub name: String,
    pub type_: String,
    pub running: bool,
}

#[derive(Debug, Clone)]
pub struct Session {
    pub name: String,
    pub type_: String,
    pub components: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub sessions: Vec<Session>,
    pub components: Vec<Component>,
}

impl Default for Model {
    fn default() -> Self { Self { sessions: vec![], components: vec![] } }
}

impl Model {
    pub fn new() -> Self { Self::default() }

    pub fn add_session(&mut self, name: &str, type_: &str, components: &[String]) {
        self.sessions.push(Session { name: name.into(), type_: type_.into(), components: components.to_vec() });
    }

    pub fn add_component(&mut self, name: &str, type_: &str) {
        self.components.push(Component { name: name.into(), type_: type_.into(), running: false });
    }

    pub fn start_component(&mut self, name: &str) -> bool {
        if let Some(c) = self.components.iter_mut().find(|c| c.name == name) {
            c.running = true;
            true
        } else { false }
    }

    pub fn stop_component(&mut self, name: &str) -> bool {
        if let Some(c) = self.components.iter_mut().find(|c| c.name == name) {
            c.running = false;
            true
        } else { false }
    }

    pub fn list(&self) -> &[Component] { &self.components }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn session_components() {
        let mut c = Model::new();
        c.add_session("cinnamon", "x11", &["muffin".into(), "cinnamon".into()]);
        c.add_component("muffin", "wm");
        assert!(c.start_component("muffin"));
        assert!(c.stop_component("muffin"));
    }
}
