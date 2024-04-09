import { createContext, provide } from '@lit/context';
import { Identity } from 'js/dto/identity';
import { VeritaService } from 'js/services/verita';
import { LitElement, html } from 'lit';
import { customElement, property, state } from 'lit/decorators.js';

export interface VeritaOptions {
  endpoint: string;
  useGateSlot: boolean;
}

export const veritaGateContext = createContext<VeritaGate>('verita-gate');

@customElement('verita-gate')
export class VeritaGate extends LitElement {
  @property({ attribute: false })
  options: VeritaOptions;

  @provide({ context: veritaGateContext })
  veritaGate = this;

  @state()
  identity = null;

  async connectedCallback(): Promise<void> {
    super.connectedCallback();
    const { options } = this;

    try {
      const identity = await VeritaService.init(options, 1);

      this.loggedIn(identity);
    } catch (err) {
      this.loggedOut();
    }
  }

  loggedIn(identity: Identity) {
    this.identity = identity;

    this.dispatchEvent(
      new CustomEvent('identity-change', { detail: identity }),
    );
  }

  loggedOut() {
    this.identity = null;

    this.dispatchEvent(
      new CustomEvent('identity-change', { detail: null }),
    );
  }

  render(): unknown {
    const { identity, options } = this;
    const { useGateSlot = true } = options;

    return [
      identity !== null || !useGateSlot
        ? html`
          <slot></slot>
        `
        : html`
          <slot name="gate"></slot>
        `,
    ];
  }
}
