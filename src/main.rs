use std::env;
use std::io::{self, Write};
use std::process::Command;
use colored::Colorize;

fn handle_cd(args: &[&str]) {
    if args.is_empty(){
        println!("{}", "Error: path dir missing".red());
        return;
    }
    if args.len() > 1 {
        println!("{}", "Error: Too many arguments".red());
        return ;
    }
    match args[0]{
        "~" => {
            if let Some(home) = env::var_os("HOME"){
                if let Err(_e) = env::set_current_dir(&home){
                    println!("{}", "Error: Failed to set home directory".red());
                    return ;
                }
            }
        }
        "-" => {
            if let Some(old_dir) = env::var_os("OLDPWD") {
                let curr_dir = env::current_dir().unwrap();
                unsafe {
                    env::set_var("OLDPWD", &curr_dir);
                }
                if let Err(_e) = env::set_current_dir(&old_dir) {
                    println!("{}", "Error: Failed to change directory".red());
                    return;
                }
                unsafe {
                    env::set_var("PWD", &old_dir);
                }
                println!("{}", old_dir.display()); // Optional: print the new directory like standard shells
            } else {
                println!("{}", "Error: OLDPWD not set".red());
            }
        }
        ".." => {
            let dir = env::current_dir().unwrap();
            if let Some(parent) = dir.parent(){
                if let Err(_e) = env::set_current_dir(parent){
                    println!("{}", "Error: Failed to change directory".red());
                    return ;
                }
            }
        }
        &_ => {
            let new_dir = env::current_dir().unwrap().join(args[0]);
            let canonicalized = new_dir.canonicalize().map_err(|_| {
                println!("{}", "Error: Invalid path".red());
                return;
            }).unwrap();
            if let Err(_e) = env::set_current_dir(&canonicalized) {
                println!("{}", "Error: Failed to change directory".red());
                return;
            }
        }
    }
    
}
fn main() {

    loop{
        print!("myshell> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }
        let dir = env::current_dir().unwrap();
        let command = parts[0];
        let args = &parts[1..];
        
        match command{
            "exit" => {
                break;
            }
            "cd" => {
                handle_cd(args);
                continue;
            }

            "pwd" => {
                println!("{}", dir.display());
                continue;
            }
            _ => {}
        }
        let child = Command::new(command)
            .args(args)
            .spawn();
        match child {
            Ok(mut process) => {
                process.wait().expect("Failed to wait on child");
            }
            Err(e) => {
                eprintln!("Error executing command: {}", e);
            }
        }
    }
}
