## Usage

`install <version>`

Install the Minecraft version the server should run on.

`create <name>`

Create the server folder and sign eula.txt automatically.

`start <name> [flags]`

Start the server and allocate how much memory can be used.

## Compiling

```bash
# Linux/Mac
go build -o mcserver ./src

# Windows
go build -o mcserver.exe ./src
```

## Development

```bash
go run ./src
```
