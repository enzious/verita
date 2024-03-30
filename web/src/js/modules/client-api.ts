import { createContext } from '@lit/context';
import axios, { AxiosInstance, AxiosRequestHeaders } from 'axios';

export const clientApiContext = createContext<ClientApi>('clientApi');

export type ClientApi = AxiosInstance & {
  apiEndpoint: string;
};

export const clientApi = (
  apiEndpoint: string,
): ClientApi => {
  const api = axios.create({
    baseURL: apiEndpoint,
    headers: {
      Accept: 'application/json',
      'Content-Type': 'application/json',
    },
  }) as ClientApi;

  api.apiEndpoint = apiEndpoint;

  api.interceptors.request.use(
    (config) => ({
      ...config,
      headers: {
        ...config.headers,
      } as AxiosRequestHeaders,
    }),
    (error) => Promise.reject(error),
  );

  return api;
};
