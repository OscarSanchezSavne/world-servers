#[cfg(unix)]
use std::os::unix::process::CommandExt;

pub fn relaunch_current_process(args: &[&str]) -> ! {
    let self_path = std::env::current_exe().expect("failed to get current executable path");
    let mut command = std::process::Command::new(self_path);
    command.args(args);

    #[cfg(unix)]
    {
        let err = command.exec();
        panic!("exec failed: {}", err);
    }

    #[cfg(not(unix))]
    {
        command.spawn().expect("failed to launch replacement process");
        std::process::exit(0);
    }
}
