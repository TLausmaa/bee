use std::process::exit;

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
        let mut config = Config {
            email: EmailSettings {
                from: String::from(""),
                to: String::from(""),
                smtp_email: String::from(""),
                smtp_password: String::from(""),
                smtp_server: String::from(""),
            }
        };
        
        let file = std::fs::read_to_string(".env").unwrap();
        let lines = file.lines();

        for line in lines {
            let parts: Vec<&str> = line.split("=").collect();
            if parts.len() == 2 {
                let key = parts[0].trim();
                let value = parts[1].trim();
                if key == "from" {
                    config.email.from = String::from(value);
                } else if key == "to" {
                    config.email.to = String::from(value);
                } else if key == "smtp_server" {
                    config.email.smtp_server = String::from(value);
                } else if key == "smtp_email" {
                    config.email.smtp_email = String::from(value);
                } else if key == "smtp_password" {
                    config.email.smtp_password = String::from(value);
                }
            }
        }

        config.validate();
        return config;
    }

    fn validate(&self) {
        let mut errors = vec![];
        if self.email.from.is_empty() {
            errors.push("Missing 'from' email address");
        }
        if self.email.to.is_empty() {
            errors.push("Missing 'to' email address");
        }
        if self.email.smtp_server.is_empty() {
            errors.push("Missing 'smtp_server' email address");
        }
        if self.email.smtp_email.is_empty() {
            errors.push("Missing 'smtp_email' email address");
        }
        if self.email.smtp_password.is_empty() {
            errors.push("Missing 'smtp_password' email address");
        }
        if errors.len() > 0 {
            for error in errors {
                eprintln!("Config validation error: {}", error);
            }
            exit(1);
        }
    }
}