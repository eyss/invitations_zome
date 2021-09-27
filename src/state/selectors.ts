import { InvitationEntryInfo, InvitationStatus } from "../types";


export function isInvitationCompleted(invitation: InvitationEntryInfo) {

  return (
    invitation.invitation.invitees.length ===
    invitation.invitees_who_accepted.length
  );
}

export function getInvitationStatus(invitation: InvitationEntryInfo): InvitationStatus {
    if (invitation.invitees_who_rejected.length > 0) {
        return InvitationStatus.Rejected ;
      }
  
      if (
        invitation.invitees_who_accepted.length ===
        invitation.invitation.invitees.length
      ) {
        return InvitationStatus.Completed;
      }
  
      return InvitationStatus.Pending;
}