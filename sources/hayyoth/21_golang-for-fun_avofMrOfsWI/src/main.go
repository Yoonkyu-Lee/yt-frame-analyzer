// Reconstructed from frames (87:24). The package line and imports above
// "example.com/socket" were never fully on screen (flag/log/net/http are
// inferred from use). socket/client.go and
// socket/hub.go are the gorilla/websocket examples/chat files (ServeWs exported
// and changed to return *Client, writePump renamed WritePump).
// As last seen, line 14 still points at the deleted `handler` (editor shows an
// error); whether he removed it before the final test is not visible.
package main

import (
	"flag"
	"log"
	"net/http"

	"example.com/socket"
)

var client *socket.Client

func main() {
	flag.Parse()
	hub := socket.NewHub()
	go hub.Run()

	fs := http.FileServer(http.Dir("./public"))
	http.Handle("/web/", http.StripPrefix("/web/", fs))

	http.HandleFunc("/api/send", handler)
	http.HandleFunc("/ws", func(w http.ResponseWriter, r *http.Request) {
		client = socket.ServeWs(hub, w, r)
	})

	log.Fatal(http.ListenAndServe(":8080", nil))
}
