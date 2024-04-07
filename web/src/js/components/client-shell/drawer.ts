import { LitElement, html } from 'lit';
import { customElement } from 'lit/decorators.js';

import 'fuzionkit/tree';

@customElement('verita-drawer')
class _VeritaDrawer extends LitElement {
  render(): unknown {
    return html`
      <fzn-tree>
        <div slot="top">
        </div>
      </fzn-tree>
    `;
  }
}
