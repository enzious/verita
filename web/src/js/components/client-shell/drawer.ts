import { LitElement, html } from 'lit';
import { customElement } from 'lit/decorators.js';

import 'fuzionkit/tree';
import 'fuzionkit/inputs/select/select.js';

import styles from './drawer.lit.scss?lit';

@customElement('verita-drawer')
class _VeritaDrawer extends LitElement {
  static styles = [ styles ];

  render(): unknown {
    return html`
      <fzn-tree>
        <div slot="top">
          <fzn-form-group
            label="Realm"
          >
            <fzn-select
              .options=${[
                {
                  key: 'operator',
                  label: 'Operator',
                },
              ]}
            ></fzn-select>
          </fzn-form-group>
        </div>
      </fzn-tree>
    `;
  }
}
