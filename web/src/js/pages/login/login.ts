import { EnhancedEventTargetMixin } from 'fuzionkit/utils/events.js';
import { LitElement, html } from 'lit';
import { customElement } from 'lit/decorators.js';

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
