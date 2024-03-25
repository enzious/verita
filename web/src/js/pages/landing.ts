import { EnhancedEventTargetMixin } from 'fuzionkit/utils/events.js';
import { LitElement, html } from 'lit';
import { customElement } from 'lit/decorators.js';

import 'js/components/loader/loader';

@customElement('verita-landing')
export class Verita extends EnhancedEventTargetMixin<typeof LitElement, Verita>(LitElement) {
  render(): unknown {
    return html`
      <span style="color: white;">
        <verita-loader></verita-loader>
      </span>
    `;
  }
}
