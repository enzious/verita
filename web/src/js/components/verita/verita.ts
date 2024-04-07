import { createContext, provide } from '@lit/context';
import { instill } from 'fuzionkit/context/instill.js';
import { VeritaService } from 'js/services/verita';
import { LitElement, html } from 'lit';
import { customElement, property, state } from 'lit/decorators.js';

export interface VeritaOptions {
  endpoint: string;
  useGateSlot: boolean;
}

export const veritaGateContext = createContext<VeritaGate>('verita-gate');
export const authenticatedContext = createContext<boolean>('verita-authenticated');

@customElement('verita-gate')
export class VeritaGate extends LitElement {
  @property({ attribute: false })
  options: VeritaOptions;

  @provide({ context: veritaGateContext })
  veritaGate = this;

  @instill({ context: authenticatedContext })
  @provide({ context: authenticatedContext })
  @state()
  authenticated = false;

  async connectedCallback(): Promise<void> {
    super.connectedCallback();
    const { options } = this;

    try {
      await VeritaService.init(options, 1);

      this.authenticated = true;

      this.dispatchEvent(
        new CustomEvent('authenticated-change', { detail: true }),
      );
    } catch (err) {
      this.dispatchEvent(
        new CustomEvent('authenticated-change', { detail: false }),
      );
    }
  }

  loggedIn() {
    this.authenticated = true;

    this.dispatchEvent(
      new CustomEvent('authenticated-change', { detail: true }),
    );
  }

  loggedOut() {
    this.authenticated = false;

    this.dispatchEvent(
      new CustomEvent('authenticated-change', { detail: false }),
    );
  }

  render(): unknown {
    const { authenticated, options } = this;
    const { useGateSlot = true } = options;

    return [
      authenticated || !useGateSlot
        ? html`
          <slot></slot>
        `
        : html`
          <slot name="gate"></slot>
        `,
    ];
  }
}
