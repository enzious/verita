import { LitElement, html } from 'lit';
import { customElement, property } from 'lit/decorators.js';

import styles from './basic-page.lit.scss?lit';
import { styleMap } from 'lit/directives/style-map.js';

@customElement('fzn-page-basic')
export class PageBasic extends LitElement {
  static styles = [ styles ];

  @property({ attribute: true, type: String })
  maxWidth: string | undefined;

  render(): unknown {
    const { maxWidth } = this;

    return html`
      <slot name="header"></slot>
      
      <div
        class="body"
        style=${styleMap({
          maxWidth,
        })}
      >
        <slot></slot>
      </div>
    `;
  }
}
