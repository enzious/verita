import { EnhancedEventTargetMixin } from 'fuzionkit/utils/events.js';
import { LitElement, html } from 'lit';
import { customElement } from 'lit/decorators.js';
import { consume } from '@lit/context';

import { ClientApi, clientApiContext } from 'js/modules/client-api';
import { SessionService } from 'js/services/session';
import { VeritaGate, veritaGateContext } from 'js/components/verita/verita';

import 'fuzionkit/inputs/button/button.js';
import 'js/components/loader/loader';

import styles from './landing.lit.scss?lit';

@customElement('verita-landing-page')
export class Landing extends EnhancedEventTargetMixin<typeof LitElement, Landing>(LitElement) {
  static styles = [ styles ];

  @consume({ context: clientApiContext })
  clientApi: ClientApi;

  @consume({ context: veritaGateContext })
  veritaGate: VeritaGate;

  sessionService: SessionService;

  connectedCallback(): void {
    super.connectedCallback();
    const { clientApi } = this;

    this.sessionService = new SessionService(clientApi);
  }

  handleLogoutClick = async () => {
    const { sessionService } = this;

    await sessionService.logout(1);
    this.veritaGate.loggedOut();
  };

  render(): unknown {
    const { handleLogoutClick } = this;

    return html`
      <fzn-button @click=${handleLogoutClick}>
        Logout
      </fzn-button>
    `;
  }
}
