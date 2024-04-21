import { LitElement, html } from 'lit';
import { customElement } from 'lit/decorators.js';

import 'fuzionkit/tabs';
import 'js/components/basic-page';

@customElement('verita-roles-page')
export class VeritaRoles extends LitElement {
  render(): unknown {
    return html`
      <fzn-page-basic maxwidth="800px">
        <fzn-tabs slot="header" maxwidth="800px">
          <fzn-tab active>
            <fa-icon type="fas fa-hat-wizard"></fa-icon>

            Roles
          </fzn-tab>
        </fzn-tabs>
      </fzn-page-basic>
    `;
  }
}
