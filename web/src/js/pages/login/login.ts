import { consume } from '@lit/context';
import { LitElement, html } from 'lit';
import { customElement, state } from 'lit/decorators.js';

import { ChangeEvent, EnhancedEventTargetMixin } from 'fuzionkit/utils/events.js';
import { LoginValue } from 'fuzionkit/login/login.js';
import { ClientApi, clientApiContext } from 'js/modules/client-api';
import { Realm } from 'js/dto/realm';
import { SessionService } from 'js/services/session';
import { VeritaGate, veritaGateContext } from 'js/components/verita/verita';

import 'js/components/loader/loader';
import 'fuzionkit/inputs/button/button.js';
import 'fuzionkit/tabs/tabs.js';
import 'fuzionkit/login/login.js';
import 'fuzionkit/register/register.js';
import 'fuzionkit/panel/panel.js';

import styles from './login.lit.scss?lit';

@customElement('verita-login-page')
export class Login extends EnhancedEventTargetMixin<typeof LitElement, Login>(LitElement) {
  static styles = [ styles ];

  @consume({ context: clientApiContext })
  clientApi: ClientApi;

  @consume({ context: veritaGateContext })
  veritaGate: VeritaGate;

  @state()
  realm: Realm | null = null;

  @state()
  submitting = false;

  @state()
  loginValue?: LoginValue;

  @state()
  loginError?: string;

  sessionService: SessionService;

  connectedCallback(): void {
    super.connectedCallback();
    const { clientApi } = this;

    this.sessionService = new SessionService(clientApi);

    this.initialize();
  }

  async initialize() {
    const { clientApi } = this;

    const {
      data: realm,
    } = await clientApi.post<Realm>('/views/login', { realm: 'verita' });

    this.realm = realm;
  }

  handleLoginChange = ({ detail: { value } }: CustomEvent<ChangeEvent<LoginValue>>) => {
    this.loginValue = value;
  };

  handleLoginSubmit = async ({ detail: value }: CustomEvent<LoginValue>) => {
    const { realm, sessionService, veritaGate } = this;
    const { username, password } = value;

    this.submitting = true;

    try {
      await sessionService.login(realm.id, username, password);

      veritaGate.loggedIn();
    } catch (err) {
      this.loginError = err.message;

      this.submitting = false;
    }
  };

  render(): unknown {
    const { handleLoginChange, handleLoginSubmit, loginError, loginValue, submitting } = this;

    return this.realm
      ? (
        html`
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
              </fzn-tabs>

              <fzn-switch currentPath="/login">
                <fzn-route path="/login">
                  <fzn-login
                    @change=${handleLoginChange}
                    .error=${loginError}
                    @submit=${handleLoginSubmit}
                    ?submitting=${submitting}
                    .value=${loginValue}
                  ></fzn-login>
                </fzn-route>
              </fzn-switch>
            </fzn-panel>
          </span>
        `
      )
      : null;
  }
}
