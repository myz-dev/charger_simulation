//TODO: Maybe delete own error type if it turns out the simulation does not really produce any meaningful errors.
// TOD  If it should turn out that it could produce many errors, then `thiserror` could be used to enhance the enum.
#[derive(Debug)]
pub enum Error {
    InvalidInput(String),
}
