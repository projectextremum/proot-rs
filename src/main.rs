extern crate nix;
extern crate clap;
mod bindings;
mod fsnamespace;
mod tracee;
mod cli;
mod proot;

use proot::PRoot;
use fsnamespace::FileSystemNameSpace;

fn main() {
    // step 1: CLI parsing
    let mut fs: FileSystemNameSpace = FileSystemNameSpace::new();
    cli::get_config(&mut fs);
    let mut proot: PRoot = PRoot::new(fs);

    // step 2: Start the first tracee
    proot.launch_process();

    if !proot.is_main_thread() {
        // For any tracee process we end the program here,
        // as what follows (event loop) is only for the main thread
        return;
    }

    // step 3: Configure the signal actions
    proot.prepare_sigactions();

    // step 4: Listen to and deal with tracees events
    proot.event_loop();

    println!("{:?}", proot);
}

