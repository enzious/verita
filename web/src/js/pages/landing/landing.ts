import { EnhancedEventTargetMixin } from 'fuzionkit/utils/events.js';
import { LitElement, html } from 'lit';
import { customElement } from 'lit/decorators.js';

import 'fuzionkit/inputs/button/button.js';
import 'js/components/loader/loader';

import styles from './landing.lit.scss?lit';

@customElement('verita-landing-page')
export class Landing extends EnhancedEventTargetMixin<typeof LitElement, Landing>(LitElement) {
  static styles = [ styles ];

  render(): unknown {
    return html`
    `;
  }
}
