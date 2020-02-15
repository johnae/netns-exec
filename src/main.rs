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
    let nsname = env::args().into_iter().skip(1).next().unwrap();
    let nspath = format!("/var/run/netns/{}", nsname);

    unshare(CloneFlags::CLONE_NEWNET).expect("failed");

    let nsfd = open(nspath.as_str(), OFlag::O_RDONLY, Mode::empty())
        .expect(&format!("Could not open netns file: {}", nspath));

    setns(nsfd, CloneFlags::CLONE_NEWNET).unwrap();
    // drop privs now
    setuid(Uid::current()).unwrap();
    setgid(Gid::current()).unwrap();

    let args: Vec<_> = env::args()
        .into_iter()
        .skip(2)
        .map(|arg| CString::new(arg.as_str()).unwrap())
        .collect();

    let c_args: Vec<_> = args.iter().map(|arg| arg.as_c_str()).collect();

    nix::unistd::execvp(&c_args.first().unwrap(), c_args.as_slice())
        .expect("something went wrong executing the given command, perhaps it couldn't be found?");
}
