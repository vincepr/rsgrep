# minimal cli example in rust
grep like ability to scan a textfile and return all lines that include a search-querry.

- 'cargo run -- no example-file.txt' should the following 3 lines of the example-file.txt:
```
I'm nobody! Who are you?
Are you nobody, too?
They'd banish us, you know.
```
This is just a example to quickly lookup how to parse args, read a file and parse in ENV-variables etc. Nothing more.

- toggle case sensitive search om by using the ENV:
```
./target/debug/rsgrep to example-file.txt
Are you nobody, too?
How dreary to be somebody!

CASE_INSENSITIVE=1 ./target/debug/rsgrep to example-file.txt
Are you nobody, too?
How dreary to be somebody!
To tell your name the livelong day
To an admiring bog!
```