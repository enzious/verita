import { wrapResponseError } from 'fuzionkit/utils/response.js';
import { Realm } from 'js/dto/realm';
import { ClientApi } from 'js/modules/client-api';

export class SessionService {
  clientApi: ClientApi;

  constructor(clientApi: ClientApi) {
    this.clientApi = clientApi;
  }

  async login(realmId: Realm['id'], user: string, password: string): Promise<void> {
    const { data: _ } = await wrapResponseError(this.clientApi.post<unknown>(
      'session/login',
      { user, password, realmId },
    ));
  }

  async logout(realmId: Realm['id']): Promise<void> {
    const { data: _ } = await wrapResponseError(this.clientApi.get<unknown>(
      'session/logout',
      { params: { realm: realmId } },
    ));
  }
}
