extern crate nix;
use nix::fcntl::{open, OFlag};
use nix::sched::{setns, unshare, CloneFlags};
use nix::{
    sys::stat::Mode,
    unistd::{setgid, setuid, Gid, Uid},
};
use std::env;
use std::ffi::CString;

fn main() {
    if env::args().len() < 3 {
        panic!("Please supply at least 2 arguments - the network namespace then the command (and any arguments to that command)");
    }

    let nsname = env::args().into_iter().skip(1).next().unwrap();
    unshare(CloneFlags::CLONE_NEWNET).expect("Failed to unshare network namespace");

    let nspath = format!("/var/run/netns/{}", nsname);
    let nsfd = open(nspath.as_str(), OFlag::O_RDONLY, Mode::empty())
        .expect(&format!("Could not open netns file: {}", nspath));

    setns(nsfd, CloneFlags::CLONE_NEWNET).expect("Couldn't set network namespace");
    // drop privs now - these MUST happen in the below order, otherwise
    // dropping group privileges might fail as the user privs may have
    // changed so that the user can no longer set the gid
    setgid(Gid::current()).expect("Couldn't drop group privileges");
    setuid(Uid::current()).expect("Couldn't drop user privileges");

    let args: Vec<_> = env::args()
        .into_iter()
        .skip(2)
        .map(|arg| CString::new(arg.as_str()).unwrap())
        .collect();

    let c_args: Vec<_> = args.iter().map(|arg| arg.as_c_str()).collect();

    nix::unistd::execvp(&c_args.first().unwrap(), c_args.as_slice())
        .expect("something went wrong executing the given command, perhaps it couldn't be found?");
}
