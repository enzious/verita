import { LitElement, html } from 'lit';
import { customElement, property } from 'lit/decorators.js';
import { consume } from '@lit/context';
import { ifDefined } from 'lit/directives/if-defined.js';

import { EnhancedEventTargetMixin } from 'fuzionkit/utils/events.js';
import { VeritaGate, veritaGateContext } from 'js/components/verita/verita';
import { SessionService } from 'js/services/session';
import { ClientApi, clientApiContext } from 'js/modules/client-api';
import { identityContext } from 'js/domain/identity';
import { Identity } from 'js/dto/identity';

import 'fuzionkit/shell/shell.js';
import 'fuzionkit/fa-icon/fa-icon.js';
import 'js/components/client-shell/drawer';

import styles from './client-shell.lit.scss?lit';
import userBadgeStyles from './shell-user-badge.lit.scss?lit';

@customElement('verita-client-shell')
export default class ClientShell extends EnhancedEventTargetMixin<
  typeof LitElement,
  ClientShell
>(LitElement) {
  static styles = [ styles ];

  @consume({ context: clientApiContext })
  clientApi: ClientApi;

  @consume({ context: identityContext, subscribe: true })
  identity: Identity;

  @consume({ context: veritaGateContext })
  veritaGate: VeritaGate;

  sessionService: SessionService;

  connectedCallback(): void {
    super.connectedCallback();

    const { clientApi } = this;

    this.sessionService = new SessionService(clientApi);
  }

  handleUserBadgeClick = async () => {
    const { sessionService } = this;

    await sessionService.logout(1);
    this.veritaGate.loggedOut();
  };

  render(): unknown {
    const { handleUserBadgeClick, identity } = this;

    return html`
      <fzn-shell logoText="Verita">
        <slot></slot>

        <slot slot="floating" name="floating"></slot>

        <fzn-shell-user-badge
          @click=${handleUserBadgeClick}
          slot="status"
          username=${ifDefined(identity ? identity.username : undefined)}
        >
          ${
            identity
              ? html`
                <fa-icon
                  slot="postfix-icon"
                  type="fa-solid fa-arrow-right-from-bracket"
                ></fa-icon>`
              : null
          }
        </fzn-shell-user-badge>

        <verita-drawer slot="drawer"></verita-drawer>
      </fzn-shell>
    `;
  }
}

@customElement('fzn-shell-user-badge')
class _ShellUserBadge extends LitElement {
  static styles = [ userBadgeStyles ];

  @property({ attribute: true, type: String, reflect: true })
  username?: string;

  @property({ attribute: true, type: String, reflect: true })
  caret?: 'up' | 'down';

  render(): unknown {
    const { caret, username } = this;

    return [
      html`
        <slot name="icon">
          ${
            caret
              ? html`<fa-icon type=${`fa fa-caret-${caret}`}></fa-icon>`
              : null
          }
        </slot>
      `,
      username
        ? username
        : null,
      !username
        ? html`
          <fa-icon type="far fa-hand-spock"></fa-icon>

          Login
        `
        : null,
      html`<slot name="postfix-icon"></slot>`,
    ];
  }
}
