/// Logger trait allowing for the implementation of various interchangeable logging bahaviours
pub trait Logger {
    fn log(message: &str);
}

pub struct StdErrLogger;
impl Logger for StdErrLogger {
    fn log(message: &str) {
        eprintln!("{}", message);
    }
}
