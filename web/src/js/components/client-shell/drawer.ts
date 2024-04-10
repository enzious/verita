import { LitElement, html } from 'lit';
import { customElement } from 'lit/decorators.js';
import { consume } from '@lit/context';

import { Router, routerContext } from 'fuzionkit/router/context.js';

import 'fuzionkit/tree';
import 'fuzionkit/inputs/select/select.js';

import { TreeNode } from 'fuzionkit/tree';

import styles from './drawer.lit.scss?lit';

type DrawerItem = {
  routeTo?: string;
};

@customElement('verita-drawer')
class _VeritaDrawer extends LitElement {
  static styles = [ styles ];

  @consume({ context: routerContext, subscribe: true })
  router: Router;

  nodes: TreeNode<DrawerItem>[] = [
    {
      label: 'Manage',
      data: void 0,
      weight: 0,
      open: true,
      children: [
        {
          label: 'Clients',
          weight: 0,
          data: {
            routeTo: '/manage/clients',
          },
        },
        {
          label: 'Roles',
          weight: 0,
          data: {
            routeTo: '/manage/roles',
          },
        },
        {
          label: 'Groups',
          weight: 0,
          data: {
            routeTo: '/manage/groups',
          },
        },
        {
          label: 'Sessions',
          weight: 0,
          data: {
            routeTo: '/manage/sessions',
          },
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
          weight: 0,
          data: {
            routeTo: '/settings/realms',
          },
        },
        {
          label: 'Authentication',
          weight: 0,
          data: {
            routeTo: '/settings/authentication',
          },
        },
        {
          label: 'Identity providers',
          weight: 0,
          data: {
            routeTo: '/settings/identity',
          },
        },
        {
          label: 'Federation',
          weight: 0,
          data: {
            routeTo: '/settings/federation',
          },
        },
      ],
    },
  ];

  handleItemClick = (evt: CustomEvent<TreeNode<DrawerItem>>): void => {
    const { router } = this;
    const { detail: node } = evt;
    const { routeTo } = node.data ?? {};

    router.navigate(routeTo);
  };

  handleNodeMutation = (evt: CustomEvent<TreeNode<DrawerItem>>): void => {
    const { detail: node } = evt;

    console.log('node', node);
  };

  render(): unknown {
    const { handleItemClick, handleNodeMutation, nodes } = this;

    return html`
      <fzn-tree
        .nodeChildren=${nodes}
        @item-click=${handleItemClick}
        @node-mutation=${handleNodeMutation}
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
