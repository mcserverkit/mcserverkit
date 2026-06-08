package main

import (
	"fmt"
	"os"
	"os/exec"
)

func create(name string, memory string) {
	fmt.Println("Creating server...")
	os.Mkdir(name, 0755)
	config := readConfig()

	err := os.WriteFile(name+"/eula.txt", []byte("eula=true"), 0644)
	if err != nil {
		fmt.Println(err)
		os.Exit(1)
	}

	cmd := exec.Command("java", "-jar", config.Jar)
	if memory != "" {
		cmd = exec.Command("java", "-Xms"+memory+"G", "-Xmx"+memory+"G", "-jar", "--nogui")
	}
	cmd.Dir = name
	err = cmd.Run()

	if err != nil {
		fmt.Println(err)
		os.Exit(1)
	}
}
