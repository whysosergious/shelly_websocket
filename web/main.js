import msgpack from "https://cdn.jsdelivr.net/npm/msgpack-js@0.3.0/+esm";
window.msgpack = msgpack;

// dev
window.sh = {};

function connectWebsocket(url = "/ws/") {
	const ws = new WebSocket(url);
	ws.onmessage = (msg) => {
		console.log(msg.data);
	};

	return ws;
}

window.ws = connectWebsocket();
