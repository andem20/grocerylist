import {createSlice, configureStore, PayloadAction} from '@reduxjs/toolkit';

export interface UserStoreState {
  backendUrl: string;
  userToken: string | null;
  websocket: WebSocket | null;
}

const userSlice = createSlice({
  name: 'isLoggedIn',
  initialState: {
    backendUrl: '192.168.123.31:8080',
    userToken: null,
    websocket: null,
  } as UserStoreState,
  reducers: {
    setUserToken: (state, action: PayloadAction<string | null>) => {
      state.userToken = action.payload;
    },
    createWebsocketConnection: state => {
      const ws = new WebSocket(
        `ws://${state.backendUrl}/ws?token=${state.userToken}`,
      );

      state.websocket = ws;

      ws.onopen = () => {
        // connection opened
        console.log('Connection opened');
      };

      ws.onmessage = e => {
        // a message was received
        console.log(e.data);
      };

      ws.onerror = e => {
        // an error occurred
        console.log(e.message);
      };

      ws.onclose = e => {
        // connection closed
        console.log(e.code, e.reason);
      };
    },
  },
});

export const {setUserToken, createWebsocketConnection} = userSlice.actions;

export const store = configureStore({
  reducer: userSlice.reducer,
});
