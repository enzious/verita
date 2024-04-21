import { LitElement, html } from 'lit';
import { customElement, property, state } from 'lit/decorators.js';
import { consume } from '@lit/context';

import { Router, routerContext } from 'fuzionkit/router/context.js';
import { Realm } from 'js/dto/realm';

import 'fuzionkit/tree';
import 'fuzionkit/inputs/select/select.js';

import { TreeNode } from 'fuzionkit/tree';

import styles from './drawer.lit.scss?lit';

type DrawerItem = {
  id?: string;
  parentId?: string;
  routeTo?: string;
};

const flattenTree = (nodes: TreeNode<DrawerItem>[]): TreeNode<DrawerItem>[] => {
  return nodes.flatMap((node) => (
    [ { ...node, children: undefined }, ...flattenTree(node.children ?? []) ]
  ));
};

const parseTree = (nodes: TreeNode<DrawerItem>[], root?: TreeNode<DrawerItem>): TreeNode<DrawerItem>[] => {
  const out = [];
  let branch = undefined;

  let lastLeaf = null;

  for (const node of nodes) {
    if (node.data.parentId === root?.data.id) {
      if (lastLeaf) {
        out.push({ ...lastLeaf, children: branch ? parseTree(branch, lastLeaf) : undefined });
      }

      lastLeaf = node;
      branch = undefined;
    } else {
      branch = branch ?? [];
      branch.push(node);
    }
  }

  if (lastLeaf) {
    out.push({ ...lastLeaf, children: branch ? parseTree(branch, lastLeaf) : undefined });
  }

  return out;
};

@customElement('verita-drawer')
class _VeritaDrawer extends LitElement {
  static styles = [ styles ];

  _router: Router;

  @consume({ context: routerContext, subscribe: true })
  get router(): Router {
    return this._router;
  }

  set router(router: Router) {
    if (this._router !== router) {
      const { handleRouterNavigate } = this;

      const oldValue = this._router;
      this._router = router;

      if (oldValue) {
        oldValue.removeEventListener('navigate', handleRouterNavigate);
      }
      router.addEventListener('navigate', handleRouterNavigate);

      this.requestUpdate('router', oldValue);
    }
  }

  @property({ attribute: false })
  realms?: Array<Realm>;

  @state()
  nodes: TreeNode<DrawerItem>[] = [
    {
      label: 'Manage',
      data: {
        id: 'manage',
      },
      weight: 0,
      open: true,
      children: [
        {
          label: 'Users',
          weight: 0,
          data: {
            parentId: 'manage',
            routeTo: '/manage/users',
          },
        },
        {
          label: 'Clients',
          weight: 0,
          data: {
            parentId: 'manage',
            routeTo: '/manage/clients',
          },
        },
        {
          label: 'Roles',
          weight: 0,
          data: {
            parentId: 'manage',
            routeTo: '/manage/roles',
          },
        },
        {
          label: 'Groups',
          weight: 0,
          data: {
            parentId: 'manage',
            routeTo: '/manage/groups',
          },
        },
        {
          label: 'Sessions',
          weight: 0,
          data: {
            parentId: 'manage',
            routeTo: '/manage/sessions',
          },
        },
      ],
    },
    {
      label: 'Configure',
      data: {
        id: 'configure',
      },
      weight: 0,
      open: true,
      children: [
        {
          label: 'Realms',
          weight: 0,
          data: {
            parentId: 'configure',
            routeTo: '/configure/realms',
          },
        },
        {
          label: 'Authentication',
          weight: 0,
          data: {
            parentId: 'configure',
            routeTo: '/configure/authentication',
          },
        },
        {
          label: 'Identity providers',
          weight: 0,
          data: {
            parentId: 'configure',
            routeTo: '/configure/identity-providers',
          },
        },
        {
          label: 'Federation',
          weight: 0,
          data: {
            parentId: 'configure',
            routeTo: '/configure/federation',
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
    const { nodes } = this;
    const { detail: node } = evt;
    const { data: { id } } = node;

    const flat = flattenTree(nodes);

    const idx = flat.findIndex(({ data: { id: iId } }) => iId === id);
    if (idx !== -1) {
      flat[idx] = { ...flat[idx], ...node };
    }

    this.nodes = parseTree(flat);
  };

  handleRouterNavigate = ({ detail: path }: CustomEvent<string>): void => {
    const { nodes = [] } = this;

    let flat = flattenTree(nodes);
    flat = flat.map((node) => {
      const { data: { routeTo } } = node;

      return { ...node, selected: !!routeTo && path.startsWith(routeTo) };
    });
    this.nodes = parseTree(flat);
  };

  render(): unknown {
    const { handleItemClick, handleNodeMutation, nodes, realms } = this;

    const realmOptions = (realms ?? [])
      .map(({ id, name }) => ({ key: id, label: name }));

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
              .options=${realmOptions}
            ></fzn-select>
          </fzn-form-group>
        </div>
      </fzn-tree>
    `;
  }
}
