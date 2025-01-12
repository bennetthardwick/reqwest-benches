package main

import (
	"fmt"
	"io"
	"net/http"
	"os"
	"time"
)

func main() {
	resp, err := http.Get("http://localhost:3000")

	if err != nil {
		fmt.Println("http.Get returned error", err)
		os.Exit(1)
	}

	buf := make([]byte, 8*1024)

	start := time.Now()
	byte_count := 0

	fmt.Println("Starting go...")

	for {
		amt, err := resp.Body.Read(buf)

		if err != nil {
			if err == io.EOF {
				break
			}

			fmt.Println("resp.Body.Read returned error:", err)
			os.Exit(1)
		}

		byte_count += amt
	}

	if byte_count != 16*1024*1024*1024 {
		fmt.Println("Expected byte count to be 16 GiB but got", byte_count)
		os.Exit(1)
	}

	elapsed_secs := time.Now().Sub(start).Seconds()

	fmt.Println("go ran at an average of", float64(byte_count)/1024/1024/elapsed_secs, "MiB/s")

}
