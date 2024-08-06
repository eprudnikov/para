pub struct Context {
    pub base_directory: String,
    pub project_directory: String,
}

impl Context {
    pub fn new() -> Self {
        Context {
            base_directory: String::from("/Users/eprudnikov/second-brain"),
            project_directory: String::from("/Users/eprudnikov/second-brain/1. Projects"),
        }
    }
}

