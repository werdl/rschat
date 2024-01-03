# rschat
## A simple chat application between two users written in Rust
- Truth be told, I only wrote this project to learn the `warp` and `rusqlite` frameworks.
- Honestly, it was way easier than I expected, and development is faster than with Python, see [this project](http://github.com/dispatch-x/api)
## How to run
- This was built to be easily run, so:
```bash
make setup
make run && make clean
```
- the above commands require `make`, `sqlite3` and the `rust` toolchain with linker to be installed