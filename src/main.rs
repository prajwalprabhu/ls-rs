use std::env;
use std::fs;
use std::os::unix::fs::PermissionsExt;

#[derive(Clone, Debug)]
struct Ls {
    command: String,
    args: Vec<String>,
    hidden: bool,
    full: bool,
}
impl Ls {
    pub fn new() -> Ls {
        let args = env::args().collect();
        let command = ".".to_string();
        return Ls {
            args,
            command,
            hidden: false,
            full: false,
        };
    }
    fn init(&mut self) {
        if self.args.len() > 1 {
            if self.args[1] == "-a" {
                self.hidden = true;
                if self.args.len() > 2 {
                    self.command = self.args[2].clone();
                }
            } else if self.args[1] == "-l" {
                self.full = true;
                if self.args.len() > 2 {
                    self.command = self.args[2].clone();
                }
            } else if self.args[1] == "-la" {
                self.hidden = true;
                self.full = true;
                if self.args.len() > 2 {
                    self.command = self.args[2].clone();
                }
            } else {
                self.command = self.args[1].clone();
            }
        }
    }
    fn run(&mut self) {
        let res = fs::read_dir(self.command.clone());
        let result: fs::ReadDir;
        if res.is_ok() {
            result = res.unwrap();
        } else {
            println!("{:?}", res.err());
            return;
        }
        for e in result {
            let entry = e.ok().unwrap();
            let filename = entry.file_name().into_string().expect("Failed");
            let hide = filename.clone().as_bytes()[0].eq(&".".as_bytes()[0]);
            let metadata = entry.metadata().expect("Failed to get metadata");
            if !self.hidden && hide {
                continue;
            } else if self.full == true {
                if metadata.is_dir() {
                    println!(
                        "{}{}         {}{}",
                        termion::color::Fg(termion::color::Yellow),
                        self.get_mode(metadata.permissions().mode()),
                        filename,
                        termion::color::Fg(termion::color::Reset)
                    );
                } else {
                    println!(
                        "{}{}          {}{}",
                        termion::color::Fg(termion::color::Blue),
                        self.get_mode(metadata.permissions().mode()),
                        filename,
                        termion::color::Fg(termion::color::Reset)
                    );
                }
            } else {
                if metadata.is_dir() {
                    println!("{}{}{}", termion::color::Fg(termion::color::Yellow), filename,termion::color::Fg(termion::color::Reset));
                } else {
                    println!("{}{}{}", termion::color::Fg(termion::color::Blue), filename,termion::color::Fg(termion::color::Reset));
                }
            }
        }
    }
    fn get_mode(&self, y: u32) -> String {
        let y = format!("{:o}", y);
        let mut output = String::new();
        let y = y.as_bytes();
        if y[0] == b'4' {
            output.push('d');
            for i in 2..5 {
                output.push_str(&self.octa_to_string(y[i]));
            }
        } else if y[0] == b'1' {
            output.push('-');
            for i in 3..6 {
                output.push_str(&self.octa_to_string(y[i]));
            }
        }
        output
    }
    fn octa_to_string(&self, i: u8) -> String {
        match i {
            55 => return "rwx".to_string(),
            54 => return "rw-".to_string(),
            53 => return "r-x".to_string(),
            52 => return "r--".to_string(),
            51 => return "-wx".to_string(),
            50 => return "-w-".to_string(),
            49 => return "--x".to_string(),
            _ => return "error".to_string(),
        };
    }
}

fn main() {
    let mut ls = Ls::new();
    ls.init();
    ls.run();
}
