import {createSlice, configureStore, PayloadAction} from '@reduxjs/toolkit';
import {websocketMiddleware} from '../middleware/ws.middleware';

export interface List {
	id: string;
	title: string;
	user_id: string;
	items: {
		[key: string]: Item;
	};
}

export interface Item {
	id: string;
	list_id: string;
	name: string;
	done: boolean;
	lat: number | null;
	lng: number | null;
	cluster: number | null;
	category: string | null;
}

export interface Category {
	id: string;
	name: string;
}

export interface UserStoreState {
	backendUrl: string;
	userToken: string | null;
	lists: {
		[key: string]: List;
	};
	categories: Category[];
}

const userSlice = createSlice({
	name: 'isLoggedIn',
	initialState: {
		backendUrl: '192.168.123.31:8080',
		userToken: null,
		lists: {},
	} as UserStoreState,
	reducers: {
		setUserToken: (state, action: PayloadAction<string | null>) => {
			state.userToken = action.payload;
		},
		setLists: (state, action: PayloadAction<List[]>) => {
			action.payload.forEach(list => {
				state.lists[list.id] = list;
				state.lists[list.id].items = {};
			});
		},
		updateListItem: (state, action: PayloadAction<Item>) => {
			state.lists[action.payload.list_id].items[action.payload.id] =
				action.payload;
		},
		deleteListItem: (state, action: PayloadAction<Item>) => {
			delete state.lists[action.payload.list_id].items[action.payload.id];
		},
		clearLists: state => {
			state.lists = {};
		},
	},
});

export const {
	setUserToken,
	setLists,
	updateListItem,
	deleteListItem,
	clearLists,
} = userSlice.actions;

export const store = configureStore({
	reducer: userSlice.reducer,
	middleware: getDefaultMiddleware =>
		getDefaultMiddleware().concat(websocketMiddleware),
});
