use super::Keys;
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ReadableKeys {
    keys: Vec<(String, char, bool)>,
}
impl ReadableKeys {
    pub fn grab(&self) -> Keys {
        let mut vec1: Vec<char> = Vec::new();
        let mut vec2: Vec<bool> = Vec::new();
        for line in &self.keys {
            vec1.push(line.1);
            vec2.push(line.2);
        }
        Keys::create(vec1, vec2)
    }
}
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ReadableContext {
    context: Vec<(String, Vec<(String, Option<String>)>)>,
}
impl ReadableContext {
    pub fn get(self) -> Vec<(String, Vec<(String, Option<String>)>)> {
        self.context
    }
}
