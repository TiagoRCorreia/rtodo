use std::io::{self,Write};
use std::process::{self,Command};

use colored::Colorize;

use todos::Todo;

pub mod todos;

fn user_input(buffer:&mut String)-> Result<(),Box<dyn std::error::Error>>{
    io::stdout().flush()?;
    io::stdin().read_line(buffer)?;
    Ok(())
}

fn add_todo(td:&mut Vec<Todo>,buffer:&mut String) -> Result<(),Box<dyn std::error::Error>>{

    buffer.clear();
    print!("{}{}",
        "Title ".white().bold(),"-> ".green().bold());
    user_input(buffer)?;

    print!("{}{}",
        "Description".white().bold(),"-> ".green().bold());
    user_input(buffer)?;
     
    let mut lines = buffer.lines();
    
    td.push(Todo::new(
        lines.next()
            .ok_or("Error parsing title!")?.trim().to_string(),
        lines.next()
            .ok_or("Error parsing description!")?.trim().to_string()),
    );
    Ok(())
}

fn show_todos(todos:&Vec<Todo>){
    Command::new("clear").status().unwrap();

    println!("{:^10} {:^40} {:^40}","ID"
        .blue().bold(),"Title".blue().bold(),"Description".blue().bold());

    println!("{:-^10} {:-^40} {:-^40}",""
        .blue().bold(),"".blue().bold(),"".blue().bold());

    if !todos.is_empty(){
        for (i,x) in todos.iter().enumerate(){
            println!("{:^10} {:^40} {:^40}",
                i.to_string().blue().bold(),
                x.get_title().cyan().bold(),
                x.get_description().yellow().bold());
        }
    }
}

fn remove_todo(todos:& mut Vec<Todo>,buffer:&mut String) -> Result<(),Box<dyn std::error::Error>>{

    buffer.clear();
    print!("\n{} {}{}","ID"
        .red().bold(),"to remove "
        .white().bold(),"-> ".green().bold());
    
    user_input(buffer)?;

    println!("{}",buffer.to_string());
    let nn:usize = buffer.trim().to_string().parse::<usize>()?;
    if nn < todos.len() && !todos.is_empty(){
        todos.remove(nn);
    };
    Ok(())
}

fn update_todo(todos:&mut Vec<Todo>,buffer:&mut String) -> Result<(),Box<dyn std::error::Error>>{
    buffer.clear();

    print!("\n{} {}{}","ID"
        .red().bold(), " to update "
        .white().bold(),"-> ".green().bold());

    user_input(buffer)?;

    print!("{}{}","Title "
        .white().bold(),"-> ".green().bold());

    user_input(buffer)?;

    print!("{}{}","Description "
        .white().bold(),"-> ".white().bold());
    user_input(buffer)?;

    let mut dados = buffer.lines();
    let id = dados.next().ok_or("Error parsing ID!")?.to_string().parse::<usize>()?;
    for (i,z) in todos.into_iter().enumerate(){
        if i == id{
           z.set_title(dados.next()
                .ok_or("Error parsing title!")?.trim().to_string());

           z.set_description(dados.next()
                .ok_or("Error parsing description!")?.trim().to_string())
        }
    }
    Ok(())
}

fn main_menu(){
    println!("\n{:>5} {}\n{:>5} {}\n{:>5} {}\n{:>5} {}\n{:>5} {}\n",
        "[1]".blue(), "Show Todos".yellow(),
        "[2]".blue(), "Add Todos".yellow(),
        "[3]".blue(), "Update Todos".yellow(),
        "[4]".blue(), "Remove Todos".yellow(),
        "[5]".blue(), "Exit".yellow(),
    );
}

fn sub_menu(todos:&mut Vec<Todo>,buffer:&mut String) -> Result<bool,Box<dyn std::error::Error>>{
    print!("\n\n{} {:<12} {} {:<12} {} {:<12}",
        "[1]".blue().bold(),"Main Menu".white().bold(),
        "[2]".blue().bold(),"Add Todo".white().bold(),
        "[3]".blue().bold(),"Remove Todo.: ".white().bold(),
    );

    buffer.clear();
    user_input(buffer)?;

    match buffer.trim(){
        "1" => {show_todos(&todos); Ok(false)},
        "2" => {add_todo(todos,buffer)?; Ok(true)},
        "3" => {remove_todo(todos,buffer)?; Ok(true)},
        _ => {println!("Invalid option!"); Ok(false)},
    }
}

fn main() {

    let mut buffer = String::new();
    let mut todos: Vec<Todo> = vec![];

    loop {

        Command::new("clear").status().unwrap();
        main_menu();

        buffer.clear();

        print!("{}{}","Choose an option "
            .white().bold(),"-> ".green().bold());

        if let Err(e) = user_input(&mut buffer){
            panic!("Error reading user input!!! {}",e);
        }
       
        
        if buffer.trim().contains("1"){
            show_todos(&todos);
            loop{
                if let Ok(v) = sub_menu(&mut todos,&mut buffer){
                    if !v{
                        break; 
                    }
                }else{
                    println!("Error show submenu!!!");
                }
                show_todos(&todos);
            }
        }else if buffer.trim().contains("2"){
            if let Err(e) = add_todo(&mut todos,&mut buffer){
                println!("Error adding note!!! {}",e);
            }
        }else if buffer.trim().contains("3"){
            show_todos(&todos);
            if let Err(e) = update_todo(&mut todos,&mut buffer){
                println!("Error update note!!! {}",e)
            }
        }else if buffer.trim().contains("4"){
            show_todos(&todos);
            if let Err(e) = remove_todo(&mut todos,&mut buffer){
                println!("Error removing note!!! {}",e);
            }
        }else if buffer.trim().contains("5"){
           process::exit(0); 
        }        
    }

}
