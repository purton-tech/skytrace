use lettre::message;
use std::env;

#[derive(Clone, Debug)]
pub struct SmtpConfig {
    // Configure SMTP for email.
    pub host: String,
    pub port: u16,
    pub tls_off: bool,
    pub username: String,
    pub password: String,
    pub domain: String,
    pub from_email: message::Mailbox,
}

impl SmtpConfig {
    pub fn new() -> Option<SmtpConfig> {
        let host = env::var("SMTP_HOST");
        let username = env::var("SMTP_USERNAME");
        let password = env::var("SMTP_PASSWORD");
        let smtp_port = env::var("SMTP_PORT");
        let domain = env::var("INVITE_DOMAIN");
        let from_email = env::var("INVITE_FROM_EMAIL_ADDRESS");

        if let (Ok(host), Ok(username), Ok(password), Ok(smtp_port), Ok(domain), Ok(from_email)) =
            (host, username, password, smtp_port, domain, from_email)
        {
            Some(SmtpConfig {
                host,
                port: smtp_port.parse::<u16>().unwrap(),
                tls_off: env::var("SMTP_TLS_OFF").is_ok(),
                username,
                password,
                domain,
                from_email: from_email.parse().unwrap(),
            })
        } else {
            None
        }
    }
}

#[derive(Clone, Debug)]
pub struct Config {
    pub database_url: String,
    // Configure SMTP for email.
    pub smtp_config: Option<SmtpConfig>,
}

impl Config {
    pub fn new() -> Config {
        let database_url = env::var("APP_DATABASE_URL").expect("APP_DATABASE_URL not set");

        Config {
            database_url,
            smtp_config: SmtpConfig::new(),
        }
    }
}
