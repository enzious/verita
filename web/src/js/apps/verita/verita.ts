import { EnhancedEventTargetMixin } from 'fuzionkit/utils/events.js';
import { LitElement, css, html } from 'lit';
import { customElement, property, state } from 'lit/decorators.js';
import { SwitchRoute, buildSwitches } from 'fuzionkit/router/switch.js';
import { provide } from '@lit/context';
import { createBrowserHistory } from 'history';

import { historyContext } from 'fuzionkit/utils/history.js';
import { extract } from 'fuzionkit/context/extract.js';
import { Router, routerContext } from 'fuzionkit/router/context.js';
import { ClientApi, clientApi, clientApiContext } from 'js/modules/client-api';

import store from './store';

import 'fuzionkit/router/router.js';
import 'js/components/verita/verita';
import 'js/components/client-shell/client-shell';
import 'js/pages/landing/landing';
import 'js/pages/login/login';

const routes: SwitchRoute[] = [
  [
    '/',
    'verita-landing-page',
  ],
];

@customElement('verita-app')
export class Verita extends EnhancedEventTargetMixin<typeof LitElement, Verita>(LitElement) {
  static styles = [
    css`
      * { box-sizing: border-box; }

      :host
      {
        display: block;
        height: 100%;
      }

      :host > verita-loader
      {
        height: 100%;
        display: flex;
        justify-content: center;
      }
    `,
  ];

  @extract({ context: routerContext })
  @provide({ context: routerContext })
  @property({ attribute: false })
  router: Router;

  @provide({ context: historyContext })
  @property({ attribute: false })
  history = createBrowserHistory();

  @provide({ context: clientApiContext })
  @property({ attribute: false })
  clientApi: ClientApi;

  @state()
  authenticated = null;

  veritaOptions = {
    endpoint: '/api',
    useGateSlot: false,
  };

  constructor() {
    super();

    this.clientApi = clientApi('/api');
  }

  handleAuthenticatedChange = ({
    detail: value,
  }: CustomEvent<boolean>) => {
    this.authenticated = value;
  };

  render(): unknown {
    const { authenticated, handleAuthenticatedChange, veritaOptions } = this;

    return html`
      <verita-gate
        .options=${veritaOptions}
        @authenticated-change=${handleAuthenticatedChange}
      >
        ${
          authenticated !== null
            ? authenticated
              ? html`
                <fzn-client-shell
                  logoText="Verita"
                >
                  <fzn-router
                    .store=${store}
                  >
                    ${buildSwitches(routes)}
                  </fzn-router>
                </fzn-client-shell>
              `
              : html`
                <verita-login-page></verita-login-page>
              `
            : null
        }
      </verita-gate>
    `;
  }
}
