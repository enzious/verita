import { LitElement, html } from 'lit';
import { customElement } from 'lit/decorators.js';

import 'fuzionkit/tabs';
import 'js/components/basic-page';

@customElement('verita-authentication-page')
export class VeritaAuthentication extends LitElement {
  render(): unknown {
    return html`
      <fzn-page-basic maxwidth="800px">
        <fzn-tabs slot="header" maxwidth="800px">
          <fzn-tab active>
            <fa-icon type="fas fa-fingerprint"></fa-icon>

            Authentication
          </fzn-tab>
        </fzn-tabs>
      </fzn-page-basic>
    `;
  }
}
