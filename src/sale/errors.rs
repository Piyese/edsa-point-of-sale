use std::io;


#[derive(Debug)]
pub enum PosError{
    FileIOError(io::Error),
    SerdeYamlError(serde_yaml::Error),
}
impl From<io::Error> for PosError{
    fn from(error: io::Error)->Self{
        PosError::FileIOError(error)
    }
}
impl From<serde_yaml::Error> for PosError{
    fn from(error:serde_yaml::Error)->Self{
        PosError::SerdeYamlError(error)
    }
}