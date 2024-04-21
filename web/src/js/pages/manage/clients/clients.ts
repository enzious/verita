import { LitElement, html } from 'lit';
import { customElement } from 'lit/decorators.js';

import 'fuzionkit/tabs';
import 'js/components/basic-page';

@customElement('verita-clients-page')
export class VeritaClients extends LitElement {
  render(): unknown {
    return html`
      <fzn-page-basic maxwidth="800px">
        <fzn-tabs slot="header" maxwidth="800px">
          <fzn-tab active>
            <fa-icon type="fas fa-cube"></fa-icon>

            Clients
          </fzn-tab>
        </fzn-tabs>
      </fzn-page-basic>
    `;
  }
}
