import axios from 'axios';
import { wrapResponseError } from 'fuzionkit/utils/response.js';
import { VeritaOptions } from 'js/components/verita/verita';

const init = async (options: VeritaOptions, realmId: number): Promise<void> => {
  const { endpoint } = options;

  const { data: _ } = await wrapResponseError(axios.get<unknown>(
    `${endpoint}/session/init`,
    { params: { realm: realmId } },
  ));
};

export const VeritaService = {
  init,
};
