pub struct Argument(pub String, pub String);

impl Argument {
    pub fn new(k: String, v: String) -> Self {
        Self(k, v)
    }

    pub fn key(&self) -> &String {
        &self.0
    }

    pub fn value(&self) -> &String {
        &self.1
    }
}
