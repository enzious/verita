import { EnhancedEventTargetMixin } from 'fuzionkit/utils/events.js';
import { LitElement, css, html } from 'lit';
import { customElement, property } from 'lit/decorators.js';
import { SwitchRoute, buildSwitches } from 'fuzionkit/router/switch.js';
import { provide } from '@lit/context';
import { historyContext } from 'fuzionkit/utils/history.js';
import { createBrowserHistory } from 'history';
import { extract } from 'fuzionkit/context/extract.js';
import { Router, routerContext } from 'fuzionkit/router/context.js';

import store from './store';

import 'js/pages/landing';
import 'js/pages/login/login';
import 'fuzionkit/router/router.js';

export const routes: SwitchRoute[] = [
  [
    '/',
    'verita-login-page',
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

  render(): unknown {
    // return html`<verita-loader></verita-loader>`;

    return html`
      <fzn-router
        .store=${store}
      >
        ${buildSwitches(routes)}
      </fzn-router>
    `;
  }
}
