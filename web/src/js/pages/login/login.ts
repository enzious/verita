import { consume } from '@lit/context';
import { LitElement, html } from 'lit';
import { customElement, state } from 'lit/decorators.js';

import { EnhancedEventTargetMixin } from 'fuzionkit/utils/events.js';
import { ClientApi, clientApiContext } from 'js/modules/client-api';

import 'js/components/loader/loader';
import 'fuzionkit/inputs/button/button.js';
import 'fuzionkit/tabs/tabs.js';
import 'fuzionkit/login/login.js';
import 'fuzionkit/register/register.js';
import 'fuzionkit/panel/panel.js';

import styles from './login.lit.scss?lit';
import { Realm } from 'js/dto/realm';

@customElement('verita-login-page')
export class Login extends EnhancedEventTargetMixin<typeof LitElement, Login>(LitElement) {
  static styles = [ styles ];

  @consume({ context: clientApiContext })
  clientApi: ClientApi;

  @state()
  realm: Realm | null = null;

  connectedCallback(): void {
    super.connectedCallback();

    this.initialize();
  }

  async initialize() {
    const { clientApi } = this;

    await clientApi.post<Realm>('/views/login', { realm: 'verita' });
  }

  render(): unknown {
    return html`
      <div class="stage">
        <verita-loader></verita-loader>

        <fzn-panel foggedglass="" style="width: 420px;">
          <fzn-tabs
            defaultValue="/login"
          >
            <fzn-tab
              key="/login"
            >
              Login
            </fzn-tab>

            <fzn-tab
              key="/register"
            >
              Register
            </fzn-tab>
          </fzn-tabs>

          <fzn-switch controlled="" currentPath="/login">
            <fzn-route path="/login">
              <fzn-login
              ></fzn-login>
            </fzn-route>

            <fzn-route path="/register">
              <fzn-register
              ></fzn-register>
            </fzn-route>
          </fzn-switch>
        </fzn-panel>
      </span>
    `;
  }
}
