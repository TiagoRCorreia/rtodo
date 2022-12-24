use std::io::{self,Write};
use std::process::{self,Command};

use colored::Colorize;

use todos::Todo;

pub mod todos;

fn user_input() -> Result<String, Box<dyn std::error::Error>>{
    let mut buffer = String::new();
    io::stdout().flush()?; 
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer)
}

/* fn user_input(buffer:&mut String)-> Result<(),Box<dyn std::error::Error>>{
    io::stdout().flush()?;
    io::stdin().read_line(buffer)?;
    Ok(())
} */

fn add_todo(td:&mut Vec<Todo>) -> Result<(),Box<dyn std::error::Error>>{

    print!("{}{}",
        "Title ".white().bold(),"-> ".green().bold());
    let title = user_input()?.trim().to_string();
     

    print!("{}{}",
        "Description".white().bold(),"-> ".green().bold());
    let desc = user_input()?.trim().to_string();

    td.push(Todo::new(title, desc));
    Ok(())

}

/* fn add_todo(td:&mut Vec<Todo>,buffer:&mut String) -> Result<(),Box<dyn std::error::Error>>{

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
} */


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
                x.title.cyan().bold(),
                x.description.yellow().bold());
        }
    }
}

fn remove_todo(todos:& mut Vec<Todo>) -> Result<(),Box<dyn std::error::Error>>{

    print!("\n{} {}{}","ID"
        .red().bold(),"to remove "
        .white().bold(),"-> ".green().bold());
    

    //println!("{}",buffer.to_string());

    let rm =  user_input()?.trim().to_string().parse::<usize>()?;

    if rm < todos.len() && !todos.is_empty(){
        todos.remove(rm);
    };
    Ok(())
}

fn update_todo(todos:&mut Vec<Todo>) -> Result<(),Box<dyn std::error::Error>>{

    print!("\n{} {}{}","ID"
        .red().bold(), " to update "
        .white().bold(),"-> ".green().bold());

    let id = user_input()?.trim().to_string();
    
    print!("{}{}","Title "
        .white().bold(),"-> ".green().bold());

    let title = user_input()?.trim().to_string();

    print!("{}{}","Description "
        .white().bold(),"-> ".white().bold());
    let desc = user_input()?.trim().to_string();

    let id = id.trim().to_string().parse::<usize>()?;

    for (i,z) in todos.into_iter().enumerate(){
        if i == id{
           //z.set_title(title);
           //z.set_description(desc.trim().to_string())
            z.title = title.to_string();
            z.description = desc.to_string();
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

fn sub_menu(todos:&mut Vec<Todo>) -> Result<bool,Box<dyn std::error::Error>>{
    print!("\n\n{} {:<12} {} {:<12} {} {:<12}",
        "[1]".blue().bold(),"Main Menu".white().bold(),
        "[2]".blue().bold(),"Add Todo".white().bold(),
        "[3]".blue().bold(),"Remove Todo.: ".white().bold(),
    );

/*     buffer.clear();
    user_input(buffer)?;
    let bf = user_input()?.trim().to_string(); */

    let id = user_input()?.trim().to_string(); 

    if id.contains("1"){
        show_todos(todos);
        Ok(false)
    }else if id.contains("2"){
        add_todo(todos)?;
        Ok(true)
    }else if id.contains("3"){
        remove_todo(todos)?;
        Ok(true)
    }else{
        println!("Option Invalid!");
        Ok(false)
    }


   /*  match id{
        "1" => {show_todos(todos); Ok(false)},
        "2" => {add_todo(todos)?; Ok(true)},
        "3" => {remove_todo(todos)?; Ok(true)},
        _ => {println!("Invalid option!"); Ok(false)},
    } */
}

fn main() {

    //let mut buffer = String::new();
    let mut todos: Vec<Todo> = vec![Todo{title:"testes".to_string(),description:"desctestes".to_string()}];

    loop {

        //Command::new("clear").status().unwrap();
        main_menu();

        print!("{}{}","Choose an option "
            .white().bold(),"-> ".green().bold());

        let user = match user_input(){
            Ok(u) => u.trim().to_string(),
            Err(e) => panic!("Erro get user input {e}"),
        };      

        if user.contains("1"){
            //process::Command::new("clear").status().unwrap();
            show_todos(&todos);
        }else if user.contains("2"){
            if let Err(e) = add_todo(&mut todos){
                println!("Erro add todo!!! {e}");
            }
        }else if user.contains("3"){
            if let Err(e) = update_todo(&mut todos){
                println!("Error update todo!!! {e}");
            }
        }else if user.contains("4"){
            if let Err(e) = remove_todo(&mut todos){
                println!("Error remove todo!!! {e}");
            }
        }else if user.contains("5"){
            process::exit(0);
        }

        

        
/*         if buffer.trim().contains("1"){
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
        }


        else if buffer.trim().contains("2"){
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
        }*/        
    } 

}
