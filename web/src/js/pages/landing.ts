import { EnhancedEventTargetMixin } from 'fuzionkit/utils/events.js';
import { LitElement, html } from 'lit';
import { customElement } from 'lit/decorators.js';

import 'js/components/loader/loader';

@customElement('verita-landing-page')
export class Landing extends EnhancedEventTargetMixin<typeof LitElement, Landing>(LitElement) {
  render(): unknown {
    return html`
      <span style="color: white;">
        hello
      </span>
    `;
  }
}
