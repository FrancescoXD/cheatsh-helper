# cheatsh-helper
Cheat.sh Rust Helper

## Installation
### Compilation
```
cargo build --release
```
And then it will make an executable in the `target/release` directory called `cheatsh-helper`.

Call it `ch` or something that don't exists already (check first if the command exists):
```
mv cheatsh-helper ch
```

Move it to `/usr/local/bin`:
```
sudo cp ch /usr/local/bin
```

And then just write `ch ls` to see the magic!