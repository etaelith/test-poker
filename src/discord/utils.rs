use chrono::{Duration, NaiveDate};

pub fn parse_fecha(fecha: &str) -> Result<i64, &'static str> {
    match NaiveDate::parse_from_str(fecha, "%d/%m/%Y") {
        Ok(parsed_date) => {
            let zero_seconds = Duration::zero();
            let naive_datetime = parsed_date + zero_seconds;
            let epoch_seconds = naive_datetime.and_hms_opt(0, 0, 0);
            match epoch_seconds {
                Some(time) => Ok(time.and_utc().timestamp()),
                None => Err("Fecha inválida. Asegúrate de usar el formato DD/MM/YYYY."),
            }
        }
        Err(_) => Err("Fecha inválida. Asegúrate de usar el formato DD/MM/YYYY."),
    }
}
