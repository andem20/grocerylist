import React from 'react';
import {Text, FlatList, ListRenderItemInfo, View, Button} from 'react-native';
import {useDispatch, useSelector} from 'react-redux';
import {UserStoreState, setUserToken} from '../stores/UserStore';

const lists = ['List 1', 'List 2'];

export function ListScreen() {
  const dispatch = useDispatch();

  return (
    <View>
      <FlatList data={lists} renderItem={ListItem} />
      <Button title="Logout" onPress={() => dispatch(setUserToken(null))} />
    </View>
  );
}

function ListItem({item}: ListRenderItemInfo<string>) {
  return <Text>{item}</Text>;
}
