//
// gash.rs
//
// Starting code for PS2
// Running on Rust 1+
//
// University of Virginia - cs4414 Spring 2014
// Weilin Xu, David Evans
// Version 0.4
//

extern crate getopts;

use getopts::Options;
use std::env;
use std::io::{self, Write};
use std::process::Command;
use std::path::Path;
use std::io::{Read, Write};
use std::fs::File;

struct Shell<'a> {
  cmd_prompt: &'a str,
}

impl <'a>Shell<'a> {
  // note in parameters field there is no &self, so this is a static or associated method
  fn new(prompt_str: &'a str) -> Shell<'a> {
    Shell { cmd_prompt: prompt_str }
  }
  // `&self` is sugar for `self: &Self`, where `Self` is the type of the
  // caller object.
  fn run(&self) {
    // stdin() returns a handle that is a reference to a shared global buffer
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
      stdout.write(self.cmd_prompt.as_bytes()).unwrap();
      // flush everything remained in the buffer to stdout
      // but why unwrap Result<>
      stdout.flush().unwrap();

      let mut line = String::new();

      stdin.read_line(&mut line).unwrap();
      let cmd_line = line.trim();
      // cmd_line is this form: prog [arg1 arg2 ...]
      // program extract "prog [arg1 arg2 ...]"and panic with "no program" if Err
      let program = cmd_line.splitn(1, ' ').nth(0).expect("no program");
      // internal commands are implemented here
      // _ mathces external command
      match program {
        ""      =>  { continue; }
        "history" => { self.run_hist(cmd_line); }
        "exit"  =>  { return; }
        _       =>  { 
          let program = cmd_line.splitn(2, ' ').nth(0).expect("no program");
          if program == "cd" {
            self.run_cd(cmd_line);
          } else {
            self.run_cmdline(cmd_line);
          }
        }
      }
    }
  }
  // according to the bash cd, should only take args[1] as path
  // ignore the rest args if any
  fn run_cd(&self, cmd_line: &str) {
    let args: Vec<&str> = cmd_line.split(' ').filter_map(|x| {
      if x == "" {
        None
      } else {
        Some(x)
      }
    }).collect();
    
    if args.capacity() == 1 {
      ();
    } else {
      // path without "/" is considered content in the current directory
      // no need add current direct to make it complete
      let path = Path::new(&args[1]);
      let does_exist = path.exists();
      let is_path = path.is_dir();
      if does_exist && is_path {
        assert!(env::set_current_dir(&path).is_ok());
      } else {
        if args[1] == ".." {
          assert!(env::set_current_dir(path.parent().unwrap()).is_ok());
        } else {
          println!("{} doesn't exist", path.display());
        }
      }
    }
  }

  fn run_hist(&self, cmd_line: &str) {

  }

  fn run_cmdline(&self, cmd_line: &str) {
    // if None, filtered; if Some(), returned
    let argv: Vec<&str> = cmd_line.split(' ').filter_map(|x| {
      if x == "" {
        None
      } else {
        Some(x)
      }
    }).collect();
    // if None, cmd_line is empty
    match argv.first() {
      Some(&program) => self.run_cmd(program, &argv[1..]),
      None => (),
    };
  }
  // external command is external program, need a new process to execute it
  // stdout is a field of struct Output of type: Vec<u8>, take a slice of it
  // to use write method, all the output displays in the stdout of gash
  fn run_cmd(&self, program: &str, argv: &[&str]) {
    if self.cmd_exists(program) {
      io::stdout().write(&Command::new(program).args(argv).output().unwrap().stdout).unwrap();
    } else {
      println!("{}: command not found", program);
    }
  }
  // "which program" output the path to "program"
  // this execute "which" as a child process and return status as a Result<ExitStatus>
  // if success then program exists, and because by using status()
  // stdin, stdout, stderr are inherited by the child, so the return value of "which"
  // will be shown in gash
  fn cmd_exists(&self, cmd_path: &str) -> bool {
    Command::new("which").arg(cmd_path).status().unwrap().success()
  }
}

fn get_cmdline_from_args() -> Option<String> {
  /* Begin processing program arguments and initiate the parameters. */
  let args: Vec<_> = env::args().collect();
  // struct Options
  let mut opts = Options::new();
  opts.optopt("c", "", "", "");
  // remember args[0] is the prog path
  // parse() returns Result<Matches, Fail>
  // Matches is a struct
  // The result of checking command line arguments
  // Contains a vector of matches and a vector of free strings.
  // if args[1..] contains["-c", "pwd"], it's a match
  // "pwd" is returned
  opts.parse(&args[1..]).unwrap().opt_str("c")
}

fn main() {
  let opt_cmd_line = get_cmdline_from_args();
  // note if "-c CMD" is matched, no run thus no loop, just execute
  // run_cmdline and gash returns
  match opt_cmd_line {
    Some(cmd_line) => Shell::new("").run_cmdline(&cmd_line),
    None           => Shell::new("gash > ").run(),
  }
}
