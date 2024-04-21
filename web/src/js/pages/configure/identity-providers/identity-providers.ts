import { LitElement, html } from 'lit';
import { customElement } from 'lit/decorators.js';

import 'fuzionkit/tabs';
import 'js/components/basic-page';

@customElement('verita-identity-providers-page')
export class VeritaIdentityProviders extends LitElement {
  render(): unknown {
    return html`
      <fzn-page-basic maxwidth="800px">
        <fzn-tabs slot="header" maxwidth="800px">
          <fzn-tab active>
            <fa-icon type="fas fa-hands-holding-circle"></fa-icon>

            Identity Providers
          </fzn-tab>
        </fzn-tabs>
      </fzn-page-basic>
    `;
  }
}
