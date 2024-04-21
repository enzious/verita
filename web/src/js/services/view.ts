import { wrapResponseError } from 'fuzionkit/utils/response.js';
import { ShellInit } from 'js/dto/shell';
import { ClientApi } from 'js/modules/client-api';

export class ViewService {
  clientApi: ClientApi;

  constructor(clientApi: ClientApi) {
    this.clientApi = clientApi;
  }

  async initShell(): Promise<ShellInit> {
    const { data } = await wrapResponseError(this.clientApi.get<ShellInit>(
      'views/shell',
    ));

    return data;
  }
}
