import { EnhancedEventTargetMixin } from 'fuzionkit/utils/events.js';
import { LitElement, css, html } from 'lit';
import { customElement, property } from 'lit/decorators.js';
import { buildSwitches } from 'fuzionkit/router/switch.js';
import { ContextRoot, provide } from '@lit/context';
import { createBrowserHistory } from 'history';

import { historyContext } from 'fuzionkit/utils/history.js';
import { extract } from 'fuzionkit/context/extract.js';
import { Router, routerContext } from 'fuzionkit/router/context.js';
import { ClientApi, clientApi, clientApiContext } from 'js/modules/client-api';
import { Identity } from 'js/dto/identity';
import { identityContext } from 'js/domain/identity';

import { routes } from './routes';
import store from './store';

import 'fuzionkit/router/router.js';
import 'js/components/verita/verita';
import 'js/components/client-shell/client-shell';

import styles from './verita.lit.scss?lit';

@customElement('verita-app')
export class Verita extends EnhancedEventTargetMixin<typeof LitElement, Verita>(LitElement) {
  static styles = [
    styles,
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

  @provide({ context: identityContext })
  @property({ attribute: false })
  identity: Identity;

  @provide({ context: clientApiContext })
  @property({ attribute: false })
  clientApi: ClientApi;

  veritaOptions = {
    endpoint: '/api',
    useGateSlot: false,
  };

  contextRoot = new ContextRoot();

  constructor() {
    super();

    this.contextRoot.attach(this.parentElement);
    this.clientApi = clientApi('/api');
  }

  connectedCallback() {
    super.connectedCallback();
  }

  handleIdentityChange = ({
    detail: value,
  }: CustomEvent<Identity>) => {
    this.identity = value;
  };

  render(): unknown {
    const { handleIdentityChange, identity, veritaOptions } = this;

    return html`
      <verita-gate
        .options=${veritaOptions}
        @identity-change=${handleIdentityChange}
      >
        ${
          identity !== undefined
            ? identity
              ? html`
                <verita-client-shell
                  logoText="Verita"
                >
                  <fzn-router
                    .store=${store}
                  >
                    ${buildSwitches(routes)}
                  </fzn-router>
                </verita-client-shell>
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
