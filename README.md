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

```go
mcserverkit.Create(name string, eula bool)
```

- `name`: Folder name of your server
- `eula`: Passing `true` means you have read and agree to [Minecraft's EULA](https://www.minecraft.net/en-us/eula)

Start your server

```go
mcserverkit.Start(name string, memory ...string)
```

- `name`: Folder name of your server
- `memory`: (optional) Amount of memory allocated to the server, ex: 4G, 1024M
