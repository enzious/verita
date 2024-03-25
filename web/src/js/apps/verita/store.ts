import { Tuple, configureStore } from '@reduxjs/toolkit';
import { all } from 'redux-saga/effects';
import { Action, UnknownAction } from 'redux';
import createSagaMiddleware, { Saga } from 'redux-saga';

const reducer = {
  dummy: (
    state = null,
    _: Action<string>,
  ): null => state,
};

export type ReducerTypes<T extends { [key: string]: ((_state: unknown, _action: UnknownAction) => unknown) }> = {
  [K in keyof T]: T[K] extends ((_state: unknown, _action: UnknownAction) => infer R) ? R : never
};

export const rootSaga: Saga = function * root () {
  yield all([]);
};

const sagaMiddleware = createSagaMiddleware();

const store = configureStore({
  reducer,
  middleware: () => new Tuple(sagaMiddleware),
  devTools: process.env.NODE_ENV !== 'production',
});

export type RootState = ReducerTypes<typeof reducer>;
export type AppDispatch = typeof store.dispatch;

sagaMiddleware.run(rootSaga);

export default store;
