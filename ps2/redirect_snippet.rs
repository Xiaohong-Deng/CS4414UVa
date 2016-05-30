fn main() {
  // the spaces trailing and preceding ">" is not mandatory
  // once you see an ">" you know redirection is about to happen
  let cmd_both_redirection = "cat < input.txt > output.txt";
  let cmd_no_redirect = "ls -l";
  let cmd_redirect = "ls -l > log.txt";
  let cmd_redirect_no_file = "ls -l >";
  let cmd_redirect_no_cmd = "> log.txt";
  let frag_no_redirect: Vec<&str> = cmd_no_redirect.split(|p| p == '>' || p == '<').collect();
  let frag_redirect: Vec<&str> = cmd_redirect.split('>').collect();
  let frag_redirect_no_file: Vec<&str> = cmd_redirect_no_file.split('>').collect();
  let frag_redirect_no_cmd: Vec<&str> = cmd_redirect_no_cmd.split('>').collect();
  println!("size of cmd_no_redirect: {}", frag_no_redirect.capacity());
  for i in &frag_no_redirect {
    println!("{}", i);
  }
  println!("size of cmd_no_redirect: {}", frag_redirect.capacity());
  for i in &frag_redirect {
    println!("{}", i);
  }
  println!("size of cmd_redirect_no_file: {}", frag_redirect_no_file.capacity());
  for i in &frag_redirect_no_file {
    println!("{}", i);
  }
  println!("size of cmd_redirect_no_cmd: {}", frag_redirect_no_cmd.capacity());
  for i in &frag_redirect_no_cmd {
    println!("{}", i);
  }
  match cmd_both_redirection.find('>') {
    Some(pos) => {
      let left = &cmd_both_redirection[..pos];
      let right = &cmd_both_redirection[(pos+1)..];
      match right.find('<') {
        Some(pos) => {
          let output = &right[..pos];
          let input = &right[(pos+1)..];
          println!("program: {}", left.trim());
          println!("output: {}", output.trim());
          println!("input: {}", input.trim());
        }
        None => { match left.find('<') {
          Some(pos) => {
            let program = &left[..pos];
            let output = &left[(pos+1)..];
            println!("program: {}", program.trim());
            println!("output: {}", output.trim());
            println!("input: {}", right.trim());
          }
          None => {
            println!("program: {}", left.trim());
            println!("output: {}", right.trim());
          }
        }
      }
    }
  }
    None => { match cmd_both_redirection.find('<') {
      Some(pos) => {
        let program = &cmd_both_redirection[..pos];
        let input = &cmd_both_redirection[(pos+1)..];
        println!("program: {}", program.trim());
        println!("input: {}", input.trim());
      }
      None => (),
      }
    }
  }
}