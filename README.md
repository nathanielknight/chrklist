# Chrklist

The smallest, silliest calm productivity tool.


## What is this

Chrklist looks for newline-delimited checklists in a platform-appropriate
directory (per
[platform-dirs' AppDirs](https://docs.rs/platform-dirs/0.3.0/platform_dirs/struct.AppDirs.html))
and presents them to you one at a time in a TUI.

It's designed for those cases where you don't want an automation, but you might want to go through a checklist, such as

- when you start or end your workday
- before you push a commit
- before you release a project


## Usage

```
chrklst 0.1.0


chrklst presents the non-empty lines of a textfile to you one at a time
in a distraction-free TUI.

USAGE:

    chrklst               List available checklists
    chrklst <checklist>   Start presenting the given checklist
    chrklst -l
    chrklst -h            Print this help message
    chrklst --help
    chrklst -d            Print the full path of the directory where checklists are stored
    chrklst --directory
    chrklst -v            Print the version
    chrklst --version
```

## Tips

Chrklist will tell you what directory it searches, so you can edit a checklist
called `foo` with something like

```sh
$EDITOR $(chrklist -d)/foo
```

If you have a tool like [fzf](https://github.com/junegunn/fzf) you can
use it to choose a checklist with something like

```sh
chrklist $(chrklist | fzf)
```

## Name

The name "chrklist" came to be because I didn't want to use the name "checklist" for such a trivial project (and it's [already taken](https://crates.io/crates/checklist) anyway) so I changed one letter  in the name. 


## License

Chrklist is licensed under the [PolyForm Internal Use License 1.0.0](https://polyformproject.org/licenses/internal-use/1.0.0/). Can't have Jeff Bezos reaping all the rewards from my one-evening-hack of a project.
