use std::env::var;

#[derive(Debug, Clone)]
pub struct Config {
    //pub cors_url: String,
    pub db_user: String,
    pub db_password: String,
    pub db_url: String,
    pub jwt_secret: String,
    pub jwt_expires_in: String,
    pub jwt_maxage: i32,
}

impl Config {
    pub fn init() -> Config {
        let db_user = var("POSTGRES_USER").expect("POSTGRES_USER must be set");
        let db_password = var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD must be set");
        let db_url = var("DATABASE_URL").expect("DATABASE_URL must be set");
        // let cors_url = var("ALLOW_ORIGIN").unwrap_or(String::from("http://localhost:3000"));
        let jwt_secret = var("JWT_SECRET").expect("JWT_SECRET must be set");
        let jwt_expires_in = var("JWT_EXPIRED_IN").expect("JWT_EXPIRED_IN must be set");
        let jwt_maxage = var("JWT_MAXAGE")
            .map(|age| age.parse::<i32>())
            .expect("JWT_MAXAGE must be set")
            .expect("JWT_MAXAGE must be a number");
        Config {
            //cors_url,
            db_user,
            db_password,
            db_url,
            jwt_secret,
            jwt_expires_in,
            jwt_maxage,
        }
    }
}
