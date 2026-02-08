import ReconnectingWebSocket from 'reconnecting-websocket';

interface MSG_DATA {
	Number : number;
	Text : string;
	Array : [number, number];
}
interface MSG {
	protocol : number;
	data : MSG_DATA | number;
}

class WS {
	ws ?: ReconnectingWebSocket;
	connect = (obj : {
		url ?: string;
		onopen ?: (e : any) => void;
		onclose ?: (e : any) => void;
		onmessage ?: (e : MSG) => void
	}) => {
		this.ws = new ReconnectingWebSocket('./ws');
		this.ws.onopen = obj.onopen ?? null;
		this.ws.onclose = obj.onclose ?? null;
		this.ws.onmessage = (e : { data : string; }) => {
			if (obj.onmessage)
				obj.onmessage(JSON.parse(e.data))
		};
	};

	send = (msg : MSG) => this.ws?.send(JSON.stringify(msg));
	disconnect = () => this.ws?.close();
};

export default WS;
export type { MSG, MSG_DATA };