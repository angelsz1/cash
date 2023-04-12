use crate::infobar;
use signal_hook::consts::signal::*;
use signal_hook::consts::TERM_SIGNALS;
use signal_hook::iterator::exfiltrator::WithOrigin;
use signal_hook::iterator::SignalsInfo;
use std::io::{Error, Write};

pub fn handle() -> Result<(), Error> {
    let mut sigs = vec![
        SIGTSTP, SIGCONT, SIGWINCH, SIGINT,
        SIGHUP, // Application-specific action, to print some statistics.
        SIGUSR1,
    ];
    sigs.extend(TERM_SIGNALS);
    let mut signals = SignalsInfo::<WithOrigin>::new(&sigs)?;
    for info in &mut signals {
        match info.signal {
            SIGINT => {
                println!();
                infobar::show_infobar();
                std::io::stdout().flush().expect("flush failed!");
            }
            _ => {}
        }
    }
    Ok(())
}
