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

pg_ctl is a utility to initialize, start, stop, or control a PostgreSQL server.
Usage:
  pg_ctl init[db]   [-D DATADIR] [-s] [-o OPTIONS]
  pg_ctl start      [-D DATADIR] [-l FILENAME] [-W] [-t SECS] [-s]
                    [-o OPTIONS] [-p PATH] [-c]
  pg_ctl stop       [-D DATADIR] [-m SHUTDOWN-MODE] [-W] [-t SECS] [-s]

Start PostgreSQL running as the current user and with the Unix socket in the current directory
> pg_ctl -D .tmp/mydb -l logfile -o "--unix_socket_directories='$PWD'" start

Create a database
> createdb mydb

Then every other time you re-enter that shell, you can just run the part that starts the database. It will keep running until you reboot, or stop it like this:

> pg_ctl -D .tmp/mydb stop

In nix-shell : 
The diesel setup will look for a DATABASE_URL env variable
> diesel setup 
> diesel migration generate create_posts
(fill in relevant sql in the generated migrations)
> diesel migration run


References : 
- https://www.youtube.com/watch?v=ICVQ8yFhjuw&t=2234s
- 



