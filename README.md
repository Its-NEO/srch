# srch
A crossplatform search utility written for you who craves simplicity.

# Install
### Required:
[Rust](https://www.rust-lang.org/tools/install) 
[Git](https://git-scm.com/book/en/v2/Getting-Started-Installing-Git)

### Download:
```bash
git clone https://github.com/Its-NEO/srch
```

### Build it
```bash
cd srch
cargo build
```

### Add binary to your $PATH
```bash
cp target/debug/srch ~/.local/bin/
```


# Basic Usage

Search your filesystem for the file you're looking for. 

```bash
>> srch hello
./codespace/c/hello
./codespace/rust/hello_world

Searched through 29 file(s) and 27 folder(s) in 1 ms and found 2 results.
```

### Arguments
\<PATTERN\>
The pattern you want to search for 

### Options
+ -d, --depth \<DEPTH\> -- How deep do you want to dig into? \[default: 3\]
```bash
>> srch hello -d 5
./codespace/c/hello
./codespace/c/hello/out/hello_world.out
./codespace/c/hello/hello_world.c
./codespace/rust/hello_world
./codespace/rust/hello_world/hello_world.rs
./codespace/rust/hello_world/hello_world

Searched through 97 file(s) and 75 folder(s) in 6 ms and found 6 results.
```
	
+ --infile -- Search through text-based file's contents
```bash
>> srch hello --infile
./Downloads/Rust Playground_files/810c47919634786ee601.js:40760:12
./Downloads/Rust Playground_files/810c47919634786ee601.js:42391:12
./Downloads/Rust Playground_files/810c47919634786ee601.js:43932:12
./Downloads/Rust Playground.html:18:17
./Downloads/Rust Playground.html:17:284
./out.txt:79:2530
./out.txt:79:2531
./out.txt:87:2610

Searched through 29 file(s) and 27 folder(s) in 18 ms and found 8 results.
```
+ -i, --useignore -- Use ignore files to ignore certain files and folders
```bash
>> ls
codespace  Desktop  Downloads  out.txt  Packages  Snapshots  Wallpapers
>> srch hello
./codespace/c/hello
./codespace/rust/hello_world

Searched through 29 file(s) and 27 folder(s) in 1 ms and found 2 results.
>> echo "codespace" > .gitignore
>> srch hello --useignore

Searched through 26 file(s) and 7 folder(s) in 0 ms and found 0 results.
``` 
+ -a, --all -- Include hidden folders
```bash
>> srch hello --infile
./Downloads/Rust Playground_files/810c47919634786ee601.js:42421:12
./out.txt:18:6
./out.txt:22:8
./out.txt:26:11
./out.txt:14:49174
./out.txt:22:49177
./out.txt:22:49178

Searched through 29 file(s) and 27 folder(s) in 16 ms and found 7 results.
>> srch hello --infile --all
./.cache/yay/completion.cache:10:2474
./.cache/yay/completion.cache:0:24193
./.cache/yay/completion.cache:0:24194
./.cache/yay/completion.cache:2:45540
./.cache/yay/completion.cache:6:61558
./.cache/yay/completion.cache:2:64303
./.cache/yay/completion.cache:15:75386
./.cache/yay/completion.cache:13:99618
./Downloads/Rust Playground_files/810c47919634786ee601.js:42421:12
./.python_history:17:63
./.python_history:43:73
./.python_history:43:75
./.viminfo:24:197
./.viminfo:25:197
./.viminfo:36:198
./.viminfo:37:198
...

Searched through 431 file(s) and 958 folder(s) in 41 ms and found 318 results.
```
+ -h, --help -- Print help
```bash
A feature-rich search tool to find all you want.

Usage: srch [OPTIONS] <PATTERN>

Arguments:
  <PATTERN>  The pattern you want to search for

Options:
  -d, --depth <DEPTH>  How deep do you want to dig into? [default: 3]
  -f, --infile         Search through text-based files contents
  -a, --all            Search through hidden folders
  -i, --useignore      Use ignore files to ignore certain files and folders
  -h, --help           Print help
  -V, --version        Print version
```
+ -V, --version -- Print Version
```bash
srch 1.0
```
