import { SwitchRoute } from 'fuzionkit/router/switch.js';

import 'js/pages/landing/landing';
import 'js/pages/login/login';

import 'js/pages/manage/users/users';
import 'js/pages/manage/clients/clients';
import 'js/pages/manage/groups/groups';
import 'js/pages/manage/roles/roles';
import 'js/pages/manage/sessions/sessions';

import 'js/pages/configure/authentication/authentication';
import 'js/pages/configure/federation/federation';
import 'js/pages/configure/identity-providers/identity-providers';
import 'js/pages/configure/realms/realms';

export const routes: SwitchRoute[] = [
  [
    '/manage',
    [
      [
        './users',
        'verita-users-page',
      ],
      [
        './clients',
        'verita-clients-page',
      ],
      [
        './groups',
        'verita-groups-page',
      ],
      [
        './roles',
        'verita-roles-page',
      ],
      [
        './sessions',
        'verita-sessions-page',
      ],
    ],
  ],
  [
    '/configure',
    [
      [
        './authentication',
        'verita-authentication-page',
      ],
      [
        './federation',
        'verita-federation-page',
      ],
      [
        './identity-providers',
        'verita-identity-providers-page',
      ],
      [
        './realms',
        'verita-realms-page',
      ],
    ],
  ],
  [
    '/',
    'verita-landing-page',
  ],
];
