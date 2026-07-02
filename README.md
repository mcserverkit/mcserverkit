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
#include <stdbool.h>
#include "mcserverkit.h"

int main()
{
	Install("1.21.1");
	Create("MyServer", true);
	Start("MyServer", "4G");
}
```

`CMakeLists.txt`

```cmake
cmake_minimum_required(VERSION 3.16)
project(example LANGUAGES C)

include(FetchContent)

FetchContent_Declare(
	mcserverkit
	GIT_REPOSITORY https://github.com/mcserverkit/mcserverkit
	GIT_TAG v0.1.10
)
FetchContent_MakeAvailable(mcserverkit)

add_executable(example main.c)
target_link_libraries(example PRIVATE mcserverkit)
```

```bash
cmake -S . -B build -DCMAKE_BUILD_TYPE=Release
cmake --build build -j
```

## C++ Usage

`main.cpp`

```c++
#include <stdbool.h>
extern "C" {
	#include "mcserverkit.h"
}

int main()
{
	Install("1.21.1");
	Create("MyServer", true);
	Start("MyServer", "4G");
}
```

`CMakeLists.txt`

```cmake
cmake_minimum_required(VERSION 3.16)
project(example LANGUAGES CXX)

include(FetchContent)

FetchContent_Declare(
	mcserverkit
	GIT_REPOSITORY https://github.com/mcserverkit/mcserverkit
	GIT_TAG v0.1.10
)
FetchContent_MakeAvailable(mcserverkit)

add_executable(example main.cpp)
target_link_libraries(example PRIVATE mcserverkit)
```

```bash
cmake -S . -B build -DCMAKE_BUILD_TYPE=Release
cmake --build build -j
```

## API

Go needs package names before each function so you need to write `mcserverkit` to access them.

| Function | Go                                | C/C++                 |
| -------- | --------------------------------- | --------------------- |
| Install  | `mcserverkit.Install(version)`    | `Install(version)`    |
| Create   | `mcserverkit.Create(name, eula)`  | `Create(name, eula)`  |
| Start    | `mcserverkit.Start(name, memory)` | `Start(name, memory)` |

Install

- `version`: The Minecraft version the server runs on, pass "latest" to install the latest release.

Create

- `name`: Folder name of your server
- `eula`: Passing `true` means you have read and agree to [Minecraft's EULA](https://www.minecraft.net/en-us/eula)

Start

> [!NOTE]
> C doesn't support optional parameters so you must pass `NULL` if you don't want to set a maximum amount of memory

- `name`: Folder name of your server
- `memory`: (optional) Amount of memory allocated to the server, ex: 4G, 1024M

## Building

Windows:

```bash
go build -buildmode=c-shared -o mcserverkit.dll ./bindings/c
gendef mcserverkit.dll
dlltool -d mcserverkit.def -l libmcserverkit.a
```

Linux:

```bash
go build -buildmode=c-shared -o mcserverkit.so ./bindings/c
```

macOS:

```bash
go build -buildmode=c-shared -o mcserverkit.dylib ./bindings/c
```
