<img src="https://github.com/Dhravya/pyre/blob/11b56eee3ead32e104079638dade03f4b62a36d0/logo.png" alt="Pyre Logo" style="float: left; margin: 0 10px 0 0;" align="left" height="150" width="150">

## Pyre

Create, open, manage your Python projects with ease.

***
> This project was made as a joke, but somehow it's really useful, so I'm publishing it
<br><br>
# 👀 Why Pyre?

- I currently have 48 projects in Python projects directory, and had to navigate and find them. With Pyre, I just have to run one command to easily open the project, right in the editor I want. `pyre open <project_name>` does it for me.
- I'm a procrastinator, and initialising Virtual environment, creating gitignore, etc. was a pain, so I basically used my old project - [create-python-project](https://github.com/dhravya/create-python-project) and modified it to work with pyre. `pyre new project_name` does it all, really quickly.
- Whenever I installed dependencies, I forgot to add them in the requirements.txt file, so `pyre i <dependencies>` automatically updates the requirements file, very convenient for me
- It's just a tool to make Python development experience better, like Cargo does, for Rust.

#### If you found it useful and installed it, make sure to ⭐ this repository!

# ⬇️ Installation

Just run this command, make sure you have cargo installed
```
cargo install pyre
```

# 📷 Screenshots, and demonstrations

### Help Command
![pyre help](https://github.com/Dhravya/pyre/blob/11b56eee3ead32e104079638dade03f4b62a36d0/assets/help.png?raw=true)

### New Project command
![Pyre new discord_bot](https://github.com/Dhravya/pyre/blob/11b56eee3ead32e104079638dade03f4b62a36d0/assets/new_command.gif?raw=true))

> You can configure your open editor to whatever you want. For my case, I have set it to `code-insiders.cmd`. To config editor, use `pyre config-editor <open_command>`

As it says, all new project names and their paths are saved to the system default for LocalData, I've used the `dirs` crate to get location, and create a `pyre.json` in the directory.

You can also add projects manually. So the project manager part is irrespective of programming language, I've added the Pyre directory, to quickly open this project. It will search for the project (add project using `pyre add <name> [path (defaults to the current path)`])

![pyre open project_name](https://github.com/Dhravya/pyre/blob/11b56eee3ead32e104079638dade03f4b62a36d0/assets/open_command.gif?raw=true)

### To list all projects and interactively open them, 

![pyre list](https://github.com/Dhravya/pyre/blob/11b56eee3ead32e104079638dade03f4b62a36d0/assets/list_and_open.gif?raw=true)

### Installing dependencies (and automatically adding to requirements.txt)

![pyre install](https://github.com/Dhravya/pyre/blob/11b56eee3ead32e104079638dade03f4b62a36d0/assets/install_command.gif?raw=true)

# 📃 TODO
- Add (or Fix) the run command (the only way I could figure out was using `std::process::Command`, but then how will I take input etc? Asked for help, recommended to use Python interpreter...)
- The stuff inside [manager.rs](./src/manager.rs) is kinda badly implemented 
- Pip install isn't *interactive* i mean, it just prints the output *after* installation is done
- The config commands should be better, maybe under the `config` subcommand where ppl can pass args to do stuff (like get the data file location, change default editor command, etc.)
- Using my own [Readme Generator](https://github.com/dhravya/readme-generator) and [Nexxel's License generator](https://github.com/nexxeln/license-generator) to generate README.md and LICENSE easily

## Support
- Follow me on [Github](https://github.com/dhravya) , [Twitter](https://twitter.com/dhravyashah) and [Dev.to](https://dev.to/dhravya) 
- [Buy me a Kofi](https://ko-fi.com/dhravya)