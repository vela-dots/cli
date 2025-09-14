use crate::hypr;
use crate::ShellArgs;
use anyhow::Result;

pub fn run(args: ShellArgs) -> Result<()> {
    if args.show {
        print!("{}", hypr::show_ipc()?);
    } else if args.log {
        print!("{}", hypr::log_shell(args.log_rules.as_deref())?);
    } else if args.kill {
        print!("{}", hypr::kill_shell()?);
    } else if !args.message.is_empty() {
        print!("{}", hypr::ipc_call(&args.message)?);
    } else {
        hypr::launch_shell(args.daemon, args.log_rules.as_deref())?;
    }
    Ok(())
}
