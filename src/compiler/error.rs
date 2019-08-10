fn get_error(toks: &Vec<&str>) -> String {
    let mut output = String::new();

    for i in 0..(toks.len() - 4) {
        output.push_str(toks[4 + i]);
        output.push(':');
    }

    output.truncate(output.len() - 1);
    output.trim()
        .replace("__fire_", "")
        .replace("const", "immutable")
}

pub fn display(cc_output: String, errors: &str) -> i32 {
    let mut error_count: i32 = 0;

    for line in errors.lines() {
        let line = line.to_string();

        if (line.contains("error:") ||
            line.contains("note:") ||
            line.contains("warning:")) &&
            line.contains(".fire.cc") {

            let mut toks: Vec<&str> = line.split(":").collect();
            if toks[0] == "C" && cfg!(windows) {
                toks.remove(0);
            }
            let ln: i32 = toks[1].to_string().parse().unwrap();
            let error = get_error(&toks);

            let mut ln_i = 0;
            let mut now = false;

            for cc_line in cc_output.lines() {
                if ln_i == ln {
                    now = true;
                }
                if now && cc_line.starts_with("//@") {
                    let line: Vec<&str> = cc_line.split("@").collect();
                    let file: Vec<&str> = line[1].split(":").collect();

                    if file[0] == "<string>" {
                        continue;
                    }

                    let line_number = file[1].to_string();

                    let mut empty = String::new();
                    for _ in 0..line_number.len() {
                        empty.push(' ');
                    }

                    let mut indent = String::new();
                    while line[2].chars().skip(indent.len()).next() == Some(' ') {
                        indent.push(' ');
                    }

                    let cc_msg = toks[3].trim();
                    let msg = if cc_msg == "note" { "\x1b[34;1mnote" } else { "\x1b[31;1merror" };

                    let mut pointer = String::new();
                    let note;
                    if cc_msg == "note" {
                        if error.contains("declared immutable here") {
                            note = "help: make this mutable: `let mut ...`".to_string();
                        } else {
                            note = "".to_string();
                        }
                    } else {
                        error_count += 1;
                        note = "".to_string();
                    }

                    if note != "".to_string() {
                        pointer = format!("{}\x1b[33m^ {}", indent, note);
                    }

                    println!("{}: \x1b[0;1m{}\x1b[0m", msg, error);
                    println!("\x1b[33m --> \x1b[37m{}\x1b[0m", line[1]);
                    println!("\x1b[33m {} | \x1b[0m", empty);
                    println!("\x1b[33m {} | \x1b[0m{}", line_number, line[2]);
                    println!("\x1b[33m {} | {}\x1b[0m", empty, pointer);
                    break;
                }
                ln_i += 1;
            }
        }
    }

    error_count
}
