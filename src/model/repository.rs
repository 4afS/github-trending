use derive_more::Display;

#[derive(Display)]
#[display(fmt = "{}/{}", owner, name)]
pub struct Repository {
    owner: String,
    name: String,
}

impl Repository {
    pub fn new(owner: &str, name: &str) -> Self {
        Repository {
            owner: owner.to_string(),
            name: name.to_string(),
        }
    }

    pub fn path(&self) -> String {
        format!("{}/{}", self.owner, self.name)
    }
}
