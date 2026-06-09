package core

import (
	"os/exec"
	"path/filepath"
)

func Start(name string, memory string) error {
	config := ReadConfig()
	path := filepath.Join("..", config.Jar)

	cmd := exec.Command("java", "-jar", path, "--nogui")
	if memory != "" {
		cmd = exec.Command("java", "-Xms"+memory, "-Xmx"+memory, "-jar", "--nogui")
	}
	cmd.Dir = name
	err := cmd.Run()

	if err != nil {
		return err
	}

	return nil
}
