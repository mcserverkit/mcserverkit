package main

import (
	"fmt"
	"os"
	"os/exec"
)

func create(world string) {
	fmt.Println("Creating server...")
	os.Mkdir(world, 0755)
	config := readConfig()

	err := os.WriteFile(world+"/eula.txt", []byte("eula=true"), 0644)
	if err != nil {
		fmt.Println(err)
		os.Exit(1)
	}

	cmd := exec.Command("java", "-jar", config.Jar)
	cmd.Dir = world
	err = cmd.Run()

	if err != nil {
		fmt.Println(err)
		os.Exit(1)
	}
}
