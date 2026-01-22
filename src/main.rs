use clap::Parser;
use std::path::PathBuf;

use todo::model::TodoError;
use todo::store;


//Parser in derive will parse cli args into this struct
#[derive(Parser, Debug)]
#[command(name = "todo")]
#[command(about = "A small todo CLI app written in rust", long_about = None)]
struct Cli{
    #[command(subcommand)]
    command: Commands,
    #[arg(long, default_value = "todos.json")]
    file: PathBuf,
}


//define subcommands
#[derive(clap::Subcommand, Debug)]
enum Commands{
    Add{text: String},
    List,
    Done{id:u64},
    Rm{id:u64},
    ClearDone,
}

//main fn
fn main(){
    //parse
    let cli = Cli::parse();

    //1. Load
    let mut list = match store::load(&cli.file){
        Ok(l)=> l,
        Err(e) =>{
            eprintln!("Failed to load {}: {}", cli.file.display(), e);
            std::process::exit(1);
        }
    };

    //2. execute command
    let mut mutated = false;
    match cli.command{
        Commands::Add{text}=>{
            let t = list.add(text);
            println!("Added [{}]{}", t.id,t.text);
            mutated =  true;
        },
        Commands::List=>{
            if list.list().is_empty(){
                println!("No tasks found");
            }
            else{
                    for t in list.list(){
                        let status = if t.done{"x"} else {" "};
                        println!("[{}] {}: {}",status,t.id,t.text);
                    }
                }
        },
        Commands::Done{id}=> match list.mark_done(id){
            Ok(_)=>{
                println!("Marked {} as done", id);
            mutated = true;
            }
            Err(TodoError::NotFound(_))=>{
                eprintln!("N task found with id: {}", id);
                std::process::exit(2);
            }
        },
        Commands::Rm{id}=> match list.remove(id){
            Ok(_)=>{
                println!("Removed task {}",id);
                mutated = true;
            },
            Err(TodoError::NotFound(_))=>{
                eprintln!("No task found with id: {}", id);
                std::process::exit(2);
            }
            
        },
        Commands::ClearDone=>{
            let removed = list.clear_done();
            println!("Removed {} completed tasks", removed);
            if removed >0 {
                mutated = true;
            }
        },
    }

    //3. save if mutated

    if mutated{
        if let Err(e) = store::save(&cli.file, &list){
            eprintln!("Failed to save {}: {}", cli.file.display(), e);
            std::process::exit(1);
        }
    }
}
