use std::path::Path;

pub struct Build {}
impl Build {
    pub fn new() -> Self {
        unimplemented!()
    }
    pub fn compile(&self, _output: &str) {
        unimplemented!()
    }
    pub fn file<P: AsRef<Path>>(&mut self, _p: P) -> &mut Build {
        unimplemented!()
    }
    pub fn flag(&mut self, _flag: &str) -> &mut Build {
        unimplemented!()
    }
    pub fn get_compiler(&self) -> Tool {
        unimplemented!()
    }
    pub fn is_flag_supported(&self, _flag: &str) -> Result<bool, Error> {
        unimplemented!()
    }
}
pub struct Tool {}
impl Tool {
    pub fn path(&self) -> &Path {
        unimplemented!()
    }
}

#[derive(Debug)]
pub struct Error {}
