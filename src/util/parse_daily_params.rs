use std::fmt::Error;

pub fn parse_daily_params(param: String) -> Result<(String, f64), Error> {
    let account = match &param[0..1] {
        "a" => "alipay".to_string(),
        "w" => "wechat".to_string(),
        "b" => "bankofchina".to_string(),
        "i" => "icbc".to_string(),
        _ => {
            eprintln!("未知的账户类型: {}", &param[0..1]);
            return Err(Error);
        }
    };
    let amount = &param[1..];
    Ok((account, amount.parse::<f64>().unwrap()))
}
