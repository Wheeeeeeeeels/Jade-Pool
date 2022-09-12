use rustyline::error::ReadlineError;
use rustyline::Editor;

fn main() {
    // 创建一个具有默认配置选项的编辑器
    let mut repl = Editor::<()>::new();

    // 加载了一个带有历史命令的文件。如果该文件不存在，它会创建一个
    if repl.load_history(".repl_history").is_err() {
        println!("No previous history.");
    }
    // 无限循环，一直卡在这里，直到用户终止程序
    loop {
        // 要求用户输入一个命令。你可以在这里添加任何你想要的东西作为前缀
        let readline = repl.readline(">> ");

        // readline返回一个结果。然后用匹配语句来过滤这个结果
        match readline {
            Ok(line) => {
                repl.add_history_entry(line.as_str());
                println!("Line: {}", line);
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
    // 把命令保存到文件中。到现在为止，它们都储存在内存中
    repl.save_history(".repl_history").unwrap();
}
