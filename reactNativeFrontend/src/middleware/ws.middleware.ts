import {Dispatch, Middleware, MiddlewareAPI, UnknownAction} from 'redux';
import {
	Item,
	List,
	UserStoreState,
	deleteListItem,
	setLists,
	updateListItem,
} from '../stores/UserStore';

const WEBSOCKET_CONNECT = 'WEBSOCKET_CONNECT';
const WEBSOCKET_SEND = 'WEBSOCKET_SEND';
const WEBSOCKET_MESSAGE = 'WEBSOCKET_MESSAGE';

interface UrlPayload {
	url: string;
}

interface MessagePayload {
	message: string;
}

export const connectWebSocket = (url: string) =>
	<WsAction<UrlPayload>>{
		type: WEBSOCKET_CONNECT as typeof WEBSOCKET_CONNECT,
		payload: {url},
	};

export const sendWebSocketMessage = (message: string) =>
	<WsAction<MessagePayload>>{
		type: WEBSOCKET_SEND as typeof WEBSOCKET_SEND,
		payload: {message},
	};

interface WsAction<T> extends UnknownAction {
	type:
		| typeof WEBSOCKET_CONNECT
		| typeof WEBSOCKET_SEND
		| typeof WEBSOCKET_MESSAGE;
	payload: T;
}

interface Message {
	action: string;
	resource: string;
	content: any;
}

type StoreMiddleware = MiddlewareAPI<Dispatch<UnknownAction>, UserStoreState>;

interface ResponseHandler {
	SERVER: {
		CONNECT: (store: StoreMiddleware, lists: List[]) => void;
	};
	ITEM: {
		CREATE: (store: StoreMiddleware, item: Item) => void;
		UPDATE: (store: StoreMiddleware, item: Item) => void;
		DELETE: (store: StoreMiddleware, item: Item) => void;
	};
}

const createWebsocketMiddleware = (): Middleware => {
	const responseHandler: ResponseHandler = {
		SERVER: {
			CONNECT: (store, lists) => {
				store.dispatch(setLists(lists));
			},
		},

		ITEM: {
			CREATE: (store, item) => {
				store.dispatch(updateListItem(item));
			},
			UPDATE: (store, item) => {
				store.dispatch(updateListItem(item));
			},
			DELETE: (store, item) => {
				store.dispatch(deleteListItem(item));
			},
		},
	};

	let socket: WebSocket | null = null;

	const middleware: Middleware = store => next => (action: any) => {
		switch (action.type) {
			case WEBSOCKET_CONNECT: {
				const {url} = action.payload as UrlPayload;
				socket = new WebSocket(url);
				socket.onopen = () => {
					console.log(`WebSocket connected to ${url}`);
				};

				socket.onmessage = event => {
					const data: Message = JSON.parse(event.data);

					(responseHandler as any)[data.resource]?.[data.action]?.(
						store,
						data.content,
					);
				};

				break;
			}
			case WEBSOCKET_SEND: {
				const {message} = action.payload as MessagePayload;
				socket?.send(JSON.stringify(message));
				break;
			}
			default:
				return next(action);
		}
	};

	return middleware;
};

export const websocketMiddleware = createWebsocketMiddleware();
