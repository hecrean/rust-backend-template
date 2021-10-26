# Setup database and migrations : 

Using NixOS for postresql and diesel-cli :

```nix
with import <nixpkgs> {};
mkShell {
  buildInputs = [
    postgresql,
    diesel-cli
  ];
}
```

Creating this and then running nix-shell means you’ll end up with a new shell with PostgreSQL available, but not actually running. To make it work, you can do this the first time:

Create a database with the data stored in the current directory
> initdb -D .tmp/mydb

Start PostgreSQL running as the current user and with the Unix socket in the current directory
> pg_ctl -D .tmp/mydb -l logfile -o "--unix_socket_directories='$PWD'" start

Create a database
> createdb mydb

Then every other time you re-enter that shell, you can just run the part that starts the database. It will keep running until you reboot, or stop it like this:

> pg_ctl -D .tmp/mydb stop

In nix-shell : 
> diesel setup --database-url='postgres://localhost:5432/mydb'
> diesel migration generate create_posts
(fill in relevant sql in the generated migrations)
> diesel migration run


References : 
- https://www.youtube.com/watch?v=ICVQ8yFhjuw&t=2234s
- 



