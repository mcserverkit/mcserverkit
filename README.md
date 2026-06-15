# MCServerKit

A library for creating and managing Minecraft servers in Go & C/C++

## Go Usage

Install the package:

```bash
go get mcserverkit.github.io
```

`main.go`

```go
package main

import (
	"fmt"

	"mcserverkit.github.io"
)

func main() {
	err := mcserverkit.Install("1.21.1")
	if err != nil {
		fmt.Println("Error installing 1.21.1:", err)
		return
	}

	err = mcserverkit.Create("MyServer", true)
	if err != nil {
		fmt.Println("Error creating MyServer:", err)
		return
	}

	err = mcserverkit.Start("MyServer", "4G")
	if err != nil {
		fmt.Println("Error starting MyServer:", err)
		return
	}
}
```

## C Usage

`main.c`

```c
#include "mcserverkit.h"

int main()
{
	Install("1.21.1")
	Create("MyServer", 1)
	Start("MyServer", "4G")
}
```

## API

Install a server version

<details open>
  <summary>Go</summary>

```go
mcserverkit.Install(version string)
```

</details>

<details>
  <summary>C/C++</summary>

```c
Install(const char* version)
```

</details>

- `version`: The Minecraft version the server runs on, pass "latest" to install the latest release.

Create a server

<details open>
  <summary>Go</summary>

```go
mcserverkit.Create(name string, eula bool)
```

</details>

<details>
  <summary>C/C++</summary>

```c
Install(const char* name, bool eula)
```

</details>

- `name`: Folder name of your server
- `eula`: Passing `true` means you have read and agree to [Minecraft's EULA](https://www.minecraft.net/en-us/eula)

Start your server

<details open>
  <summary>Go</summary>

```go
mcserverkit.Start(name string, memory ...string)
```

</details>

<details>
  <summary>C/C++</summary>

```c
Start(const char* name, const char* memory)
```

> [!NOTE]
> In C you must pass `NULL` to memory if you don't want to allocate a maximum amount of memory

</details>

- `name`: Folder name of your server
- `memory`: (optional) Amount of memory allocated to the server, ex: 4G, 1024M
