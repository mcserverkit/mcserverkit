package main

import (
	"encoding/json"
	"fmt"
	"net/http"
	"os"
	"slices"
)

func parse(url string, target any) {
	resp, err := http.Get(url)
	if err != nil {
		fmt.Println(err)
		return
	}

	defer resp.Body.Close()

	err = json.NewDecoder(resp.Body).Decode(target)

	if err != nil {
		fmt.Println(err)
		return
	}
}

func install(version string) {
	url := "https://api.papermc.io/v2/projects/paper"

	var paper struct {
		Versions []string `json:"versions"`
	}

	parse(url, &paper)

	if version == "latest" {
		version = paper.Versions[len(paper.Versions)-1]
	} else if !slices.Contains(paper.Versions, version) {
		fmt.Println("Available versions:", paper.Versions)
		os.Exit(1)
	}
}
