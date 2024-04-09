import axios from 'axios';

import { wrapResponseError } from 'fuzionkit/utils/response.js';
import { VeritaOptions } from 'js/components/verita/verita';
import { Identity } from 'js/dto/identity';

const init = async (options: VeritaOptions, realmId: number): Promise<Identity> => {
  const { endpoint } = options;

  const { data } = await wrapResponseError(axios.get<Identity>(
    `${endpoint}/session/init`,
    { params: { realm: realmId } },
  ));

  return data;
};

export const VeritaService = {
  init,
};
