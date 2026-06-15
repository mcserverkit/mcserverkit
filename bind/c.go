package main

import (
	"C"

	"mcserverkit.github.io"
)

//export Install
func Install(version *C.char) C.int {
	err := mcserverkit.Install(C.GoString(version))
	if err != nil {
		return 1
	}
	return 0
}

//export Create
func Create(name *C.char, eula C.int) C.int {
	err := mcserverkit.Create(C.GoString(name), eula != 0)
	if err != nil {
		return 1
	}
	return 0
}

//export Start
func Start(name *C.char, memory *C.char) C.int {
	err := mcserverkit.Start(C.GoString(name), C.GoString(memory))
	if err != nil {
		return 1
	}
	return 0
}

func main() {}
