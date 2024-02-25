# 

Creating a new project: 

```bash 
cargo new <project name> --vcs=none
```

The `vcs=none` turns off automatic version control, e.g. making the project a git repository on creation. I'm not sure this is necessary, anymore, I just ran `cargo new` without the `vcs` flag, and it didn't make the project a git repo, so maybe they have changed this behavior since I last ran the command. 


Then, and this goes especially if you have multiple projects in the same parent directory, create a `.vscode/settings.json` file, and point rust-analyzer at the new project, by adding a block like 

```json
{
    "rust-analyzer.linkedProjects": [        
        "./<project name>/Cargo.toml",        
    ],
}     
```

This will get you proper syntax highlighting/linting/intellisense with that project, and any others that the `settings.json` file is pointed to. 

Then you can do a `cargo run` to run the project. 

But you can also do stuff like `cargo test -- --show-output`, or `cargo test -- --nocapture`, to run unit tests, and display `println!` statements when running them. 

But anyway, that's about what I've got. The real deal here is writing Rust code. 

# Versions 
- `dol_systems`
- `lsystems`
- `proc_lsystems`

