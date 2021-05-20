// Many thanks to https://github.com/anowell/pam-rs/blob/master/pam-sober/src/lib.rs
#[macro_use]
extern crate pam;

use pam::Authenticator;

fn main() {
    let mut authenticator =
        Authenticator::with_password("system-auth").expect("Failed to init PAM client.");
    authenticator
        .get_handler()
        .set_credentials("login", "password");
    authenticator
        .authenticate()
        .expect("Authentication failed!");
    // Now that we are authenticated, it's possible to open a sesssion:
    authenticator
        .open_session()
        .expect("Failed to open a session!");
}
