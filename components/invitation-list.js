import { __decorate } from "tslib";
import { html, css } from 'lit';
import { state } from 'lit/decorators.js';
import { MobxLitElement } from '@adobe/lit-mobx';
import { ScopedRegistryHost } from '@lit-labs/scoped-registry-mixin';
import { requestContext } from '@holochain-open-dev/context';
import { sharedStyles } from '../shared-styles';
import { InvitationItem } from './invitation-item';
import { INVITATIONS_STORE_CONTEXT } from '../types';
/**mwc-elements imports */
import { Card } from 'scoped-material-components/mwc-card';
import { CircularProgress } from 'scoped-material-components/mwc-circular-progress';
/**
 * @element invitation-list
 */
export class InvitationsList extends ScopedRegistryHost(MobxLitElement) {
    constructor() {
        super(...arguments);
        this.loaded = false;
    }
    async firstUpdated() {
        await this._store.fetchMyPendingInvitations();
        this.loaded = true;
    }
    renderPendingInvitations() {
        if (Object.entries(this._store.invitations).length === 0)
            return html `<div class="column center-content" style="flex: 1;">
        <span class="placeholder">There are no pending invitations yet</span>
      </div>`;
        return html ` <div class="flex-scrollable-parent" style="flex: 1; overflow-x:hidden;">
      <div class="flex-scrollable-container"  >
        <div class="flex-scrollable-y" style="flex: 1; overflow-x:hidden;" >
          ${Object.entries(this._store.invitations).map(element => {
            return html `<invitation-item
              .invitation_entry_hash=${element[1].invitation_entry_hash}
            >
            </invitation-item>`;
        })}
        </div>
      </div>
    </div>`;
    }
    render() {
        return html `
      <mwc-card style="flex: 1;">
        <div class="column" style="margin: 16px; flex: 1;">
          <span class="title" style="margin-bottom: 8px;">Pending Invitations</span>
          ${this.loaded
            ? this.renderPendingInvitations()
            : html `<div class="column center-content" style="flex: 1;">
                <mwc-circular-progress indeterminate></mwc-circular-progress>
              </div>`}
        </div>
      </mwc-card>
    `;
    }
}
InvitationsList.styles = [css `
    .invitations {
      padding: 1em;
      margin: 1em;
      display: block;
      overflow-y: auto;
    }
  `, sharedStyles];
InvitationsList.elementDefinitions = {
    'mwc-card': Card,
    'invitation-item': InvitationItem,
    'mwc-circular-progress': CircularProgress,
};
__decorate([
    requestContext(INVITATIONS_STORE_CONTEXT)
], InvitationsList.prototype, "_store", void 0);
__decorate([
    state()
], InvitationsList.prototype, "loaded", void 0);
//# sourceMappingURL=invitation-list.js.map