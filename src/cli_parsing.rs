pub enum Command {
    List,
    New(~str),
    Close(~str),
    Show(~str),
    HelpCommand(Option<~str>)
}

pub enum GlobalOption {
    Help
}

pub struct Data {
    command: Option<~Command>,
    options: ~[~GlobalOption]
}

pub fn parse_args(args: ~[~str]) -> Data {
    println(args.to_str());

    let cmd = match args {
        [_] => Some(~List),
        [_, ~"list", ..] => Some(~List),
        [_, ~"new", name] => Some(~New(name)),
        [_, ~"close", glob] => Some(~Close(glob)),
        [_, ~"show", glob] => Some(~Show(glob)),
        [_, ~"help", cmd] => Some(~HelpCommand(Some(cmd))),
        [_, ~"help"] => Some(~HelpCommand(None)),
        _ => None
    };

    let options = ~[];

    Data { command: cmd, options: options }
}




