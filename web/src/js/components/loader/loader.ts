import { LitElement, html } from 'lit';
import { customElement, property, state } from 'lit/decorators.js';
import { styleMap } from 'lit/directives/style-map.js';
import { unsafeHTML } from 'lit/directives/unsafe-html.js';
import { consume } from '@lit/context';

import { shellContext, Shell } from 'fuzionkit/shell/context.js';

import loader from '!!raw-loader!img/verita-logo.svg';

import styles from './loader.lit.scss?lit';

@customElement('verita-loader')
export class Loader extends LitElement {
  static styles = [ styles ];

  @consume({ context: shellContext })
  shell: Shell | null;

  @property({ attribute: true, type: Boolean, reflect: true })
  fullPage = false;

  @state()
  height = 0;

  connectedCallback(): void {
    super.connectedCallback();
    const { shell } = this;

    shell?.addEventListener('resize', this.handleResize);
    this.height = shell?.getContentFrameVisibleHeight();
  }

  disconnectedCallback(): void {
    super.disconnectedCallback();
    const { shell } = this;

    shell?.removeEventListener('resize', this.handleResize);
  }

  firstUpdated(): void {
    setImmediate(() => {
      this.handleResize();
    });
  }

  handleResize = (): void => {
    const { shell } = this;

    this.height = shell?.getContentFrameVisibleHeight();
  };

  render(): unknown {
    const { fullPage, height } = this;

    return html`
      <div style=${styleMap({ height: (fullPage && height && `${height}px`) || 'auto' })}>
        ${unsafeHTML(loader)}
      </div>
    `;
  }
}
