#[macro_use]

use pamsm::{Pam, PamError, PamFlag, PamServiceModule};

struct PamTime;

impl PamServiceModule for PamTime {
    fn authenticate(pamh: Pam, _: PamFlag, args: Vec<String>) -> PamError {
        println!("Checking with solana");
        // let hour = 4;
        // if hour == 4 {
        //     // Only allow authentication when it's 4 AM
        //     PamError::SUCCESS
        // } else {
        //     PamError::AUTH_ERR
        // }
        PamError::SUCCESS
    }
}

pamsm::pam_module!(PamTime);

