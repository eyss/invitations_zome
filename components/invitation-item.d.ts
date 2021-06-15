import { MobxLitElement } from '@adobe/lit-mobx';
/**mwc-elements imports */
import { Icon } from 'scoped-material-components/mwc-icon';
import { List } from 'scoped-material-components/mwc-list';
import { Button } from 'scoped-material-components/mwc-button';
import { ListItem } from 'scoped-material-components/mwc-list-item';
import { InvitationsStore } from '../invitations.store';
declare const InvitationItem_base: typeof MobxLitElement;
/**
 * @element invitation-item
 * @fires invitation-completed - after the invitation its accepted by all the invitees
 */
export declare class InvitationItem extends InvitationItem_base {
    _store: InvitationsStore;
    loaded: boolean;
    clicked: boolean;
    invitation_entry_hash: string;
    static styles: import("lit").CSSResultGroup;
    get invitationEntryInfo(): import("../types").InvitationEntryInfo;
    get invitationStatus(): string;
    firstUpdated(): Promise<void>;
    _rejectInvitation(): Promise<void>;
    _acceptInvitation(): Promise<void>;
    _clickHandler(): void;
    _invitationIcon(): import("lit-html").TemplateResult<1>;
    _invitationActionButtons(): import("lit-html").TemplateResult<1>;
    _invitationInviterAgent(): import("lit-html").TemplateResult<1>;
    _haveYouInteracted(): boolean;
    _invitationStatusInfo(): import("lit-html").TemplateResult<1>;
    render(): import("lit-html").TemplateResult<1> | undefined;
    static elementDefinitions: {
        'mwc-icon': typeof Icon;
        'mwc-list': typeof List;
        'mwc-button': typeof Button;
        'mwc-list-item': typeof ListItem;
    };
}
export {};
