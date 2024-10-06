pub struct Config {
    pub email: EmailSettings,
}

pub struct EmailSettings {
    pub from: String,
    pub to: String,
    pub smtp_email: String,
    pub smtp_password: String,
    pub smtp_server: String,
}

impl Config {
     pub fn read() -> Config {
        Config {
            email: EmailSettings {
                from: String::from(""),
                to: String::from(""),
                smtp_email: String::from(""),
                smtp_password: String::from(""),
                smtp_server: String::from(""),
            }
        }
    }
}