use std::{fs, process::Command};
use structopt::StructOpt;
use uuid::Uuid;

/// Commands to interact with Notebook
/// 
/// nb add --title = "Notebook Title" --content = "Notebook Content"
/// 
/// nb --edit
/// 
/// nb rm --id 1
/// 
/// nb search [--all] "this is"

#[derive(Debug, StructOpt)]
pub struct NotebookCli {
    #[structopt(subcommand)]
    subcommands: Subcommands,
}

#[derive(StructOpt, Debug)]
enum Subcommands {
    #[structopt(about = "Add new note")]
    Add(AddOpts),
    #[structopt(about = "Remove existing note")]
    Rm(RmOpts),
    #[structopt(about = "Search for note")]
    Search(SearchOpts),
}

#[derive(StructOpt, Debug)]
struct AddOpts {
    #[structopt(
        long,
        short = "t",
        long_help = "Note title",
        required_unless_one(&["edit"]),
        conflicts_with_all(&["edit"])
    )]
    title: Option<String>,

    #[structopt(
        long,
        short = "c",
        long_help = "Note content",
        required_unless_one(&["edit"]),
        conflicts_with_all(&["edit"])
    )]
    content: Option<String>,

    #[structopt(
        long,
        short = "e",
        long_help = "Edit with VIM",
        required_unless_one(&["title", "content"]),
        conflicts_with_all(&["title", "content"])
    )]
    edit: bool,
}

#[derive(StructOpt, Debug)]
struct RmOpts {
    #[structopt(
        long,
        short = "i",
        long_help = "ID of the note to be removed"
    )]
    id: String,
}

#[derive(StructOpt, Debug)]
struct SearchOpts {
    #[structopt(
        long,
        short = "a",
        long_help = "Search for both Title and Content"
    )]
    all: bool,

    #[structopt()]
    input: String,
}

#[derive(Debug)]
pub enum NotebookCliError {}

impl NotebookCli {
    pub fn run(&self) -> Result<i32, NotebookCliError> {
        match &self.subcommands {
            Subcommands::Add(add_opts) => {
                if add_opts.edit {
                    let tmp_file = format!("/tmp/note_{:}.nb", Uuid::new_v4());
                    let cmd = format!("vim {tmp_file}");

                    Command::new("/bin/sh")
                        .arg("-c")
                        .arg(&cmd)
                        .spawn()
                        .expect("ERROR: Failed to start VIM") // TODO: Handle errors
                        .wait()
                        .expect("ERROR: Editor crashed"); // TODO: Handle errors

                    let user_text = fs::read_to_string(tmp_file)
                        .expect("ERROR: Failed to read tmp_file"); // TODO: Handle errors

                    if let Some((title, content)) = user_text.split_once("\n") {
                        println!("Add with edit.. Title: {:}, Content: {:}", title, content);
                    } else {
                        println!("Invalid text! More than one line is required. (First line is the title)");
                    }
                } else {
                    println!("Add with opts {:?}", add_opts);
                }
            },
            Subcommands::Rm(rm_opts) => {},
            Subcommands::Search(search_opts) => {},
        }

        Ok(0)
    }
}

fn main() {
    let cli = NotebookCli::from_args();

    match cli.run() {
        Ok(_) => (),
        Err(err) => {
            println!("ERROR: {:?}", err);
        },
    }
}
