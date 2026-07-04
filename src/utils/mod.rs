use argon2::{
    Argon2, PasswordHash,
    password_hash::{SaltString, rand_core::OsRng},
};
use rand::RngExt;
use std::iter;

#[allow(dead_code)]
#[inline]
pub fn random_string(limit: usize) -> String {
    iter::repeat(())
        .map(|_| rand::rng().sample(rand::distr::Alphanumeric))
        .map(char::from)
        .take(limit)
        .collect()
}

pub fn verify_password(password: &str, password_hash: &str) -> Result<(), anyhow::Error> {
    let hash = PasswordHash::new(password_hash)
        .map_err(|e| anyhow::anyhow!("invalid password hash: {}", e))?;
    let result = hash.verify_password(&[&Argon2::default()], password);
    match result {
        Ok(_) => Ok(()),
        Err(_) => Err(anyhow::anyhow!("invalid password")),
    }
}

pub fn hash_password(password: &str) -> Result<String, anyhow::Error> {
    let salt = SaltString::generate(&mut OsRng);
    Ok(PasswordHash::generate(Argon2::default(), password, &salt)
        .map_err(|e| anyhow::anyhow!("failed to generate password hash: {}", e))?
        .to_string())
}

#[macro_export]
macro_rules! template {
    ($path:literal) => {
        {
            #[derive(askama::Template)]
            #[template(path = $path)]
            pub struct AnonymousTemplate {}

            AnonymousTemplate {}
        }
    };

    ($path:literal, { $( $field:ident: $val:expr ),+ $(,)?  }) => {
        {
            #[allow(non_camel_case_types)]
            #[derive(askama::Template)]
            #[template(path = $path)]
            pub struct AnonymousTemplate < $( $field : ::std::fmt::Display ),+ > {
                $( pub $field : $field , )+
            }

            AnonymousTemplate {
                $( $field : $val , )+
            }
        }
    };
}

#[macro_export]
macro_rules! render_template {
    ($path:literal) => {
        {
            use askama::Template;
            Text::Html($crate::template!($path).render().unwrap())
        }
    };

    ($path:literal, $($tail:tt)*) => {
        {
            use askama::Template;
            Text::Html($crate::template!($path, $($tail)*).render().unwrap())
        }
    };
}
