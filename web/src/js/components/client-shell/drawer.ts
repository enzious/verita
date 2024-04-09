import { LitElement, html } from 'lit';
import { customElement } from 'lit/decorators.js';

import 'fuzionkit/tree';
import 'fuzionkit/inputs/select/select.js';

import { TreeNode } from 'fuzionkit/tree';

import styles from './drawer.lit.scss?lit';

@customElement('verita-drawer')
class _VeritaDrawer extends LitElement {
  static styles = [ styles ];

  nodes: TreeNode<unknown>[] = [
    {
      label: 'Manage',
      data: void 0,
      weight: 0,
      open: true,
      children: [
        {
          label: 'Clients',
          data: void 0,
          weight: 0,
        },
        {
          label: 'Roles',
          data: void 0,
          weight: 0,
        },
        {
          label: 'Groups',
          data: void 0,
          weight: 0,
        },
        {
          label: 'Sessions',
          data: void 0,
          weight: 0,
        },
      ],
    },
    {
      label: 'Configure',
      data: void 0,
      weight: 0,
      open: true,
      children: [
        {
          label: 'Realms',
          data: void 0,
          weight: 0,
        },
        {
          label: 'Authentication',
          data: void 0,
          weight: 0,
        },
        {
          label: 'Identity providers',
          data: void 0,
          weight: 0,
        },
        {
          label: 'Federation',
          data: void 0,
          weight: 0,
        },
      ],
    },
  ];

  render(): unknown {
    const { nodes } = this;

    return html`
      <fzn-tree
        .nodeChildren=${nodes}
      >
        <div slot="top">
          <fzn-form-group
            label="Realm"
            style="margin-bottom: 0;"
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
