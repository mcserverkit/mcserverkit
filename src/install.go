package main

import (
	"encoding/json"
	"fmt"
	"net/http"
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
	PaperAPI := "https://fill.papermc.io/v3/projects/paper/versions/" + version + "/builds/latest"

	var PaperResponse struct {
		Downloads struct {
			ServerDefault struct {
				Url string `json:"url"`
			} `json:"server:default"`
		} `json:"downloads"`
	}

	parse(PaperAPI, &PaperResponse)

	fmt.Println(PaperResponse.Downloads.ServerDefault.Url)
}
