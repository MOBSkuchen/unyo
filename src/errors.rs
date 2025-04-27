#[derive(Debug)]
#[allow(dead_code)]
pub enum UnyoError {
    UiLoadFont,
    ApiReq(String, String),
    ApiReqFmt(String, String),
    ApiWeatherFmt
}

pub type UnyoResult<T> = Result<T, UnyoError>;