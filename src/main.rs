mod event_loop;
mod vt100;
mod window;

use std::io::Error;
use std::sync::{
    Arc,
    Mutex
};
use signal_hook::{
    consts::signal::{SIGWINCH, SIGINT},
    iterator::{
        SignalsInfo,
        exfiltrator::origin::WithOrigin
    }
};

use window::Window;
use event_loop::EventLoop;

fn main() -> Result<(), Error> {
    //let os_signals = vec![SIGWINCH, SIGINT];
    let os_signals = vec![SIGWINCH];
    let mut signal_info = SignalsInfo::<WithOrigin>::new(&os_signals)?;

    let window = Arc::new(Mutex::new(Window::init()));
    let _el = EventLoop::run(&window);

    for info in &mut signal_info {
        match info.signal {
            SIGWINCH => Window::set_dimensions(&window, 1, 1),
            //SIGINT => handle,
            _ => ()
        }
    }

    Ok(())
}
