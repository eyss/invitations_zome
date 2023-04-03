use hdk::prelude::*;
use invitation_integrity::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct InviteesListInput(pub Vec<AgentPubKey>);

//this struct wiil be used as an output value an will contain helpfull information for the ui
#[derive(Clone, Debug, Serialize, Deserialize, SerializedBytes)]
pub struct InvitationEntryInfo {
    pub invitation: Invite,
    pub invitation_entry_hash: EntryHash,
    pub invitation_action_hash: ActionHash,
    pub invitees_who_accepted: Vec<AgentPubKey>,
    pub invitees_who_rejected: Vec<AgentPubKey>,
}

#[hdk_extern]
fn send_invitations(invitees_list: InviteesListInput) -> ExternResult<Record> {
  let agent_pub_key: AgentPubKey = agent_info()?.agent_latest_pubkey;
  debug!("my agentpubkey: {:?}",agent_pub_key);

  let invited_agents: Vec<AgentPubKey> = invitees_list
      .0
      .clone()
      .into_iter()
      .map(|agent_pub_key| AgentPubKey::from(agent_pub_key))
      .collect();
    
    debug!("invited agents: {:?}",invited_agents);
  //let now = sys_time()?.as_seconds_and_nanos();

  //let date_time = DateTime::from_utc(NaiveDateTime::from_timestamp_opt(now.0, now.1).unwrap(), Utc);

  let invitation = Invite {
      invitees: invited_agents,
      inviter: AgentPubKey::from(agent_pub_key.clone()),
      timestamp: sys_time()?
    };
    let invitation_entry_hash: EntryHash = hash_entry(invitation.clone())?;

    let new_entry_hash = create_entry(&EntryTypes::Invite(invitation.clone()))?;
    let record = get(new_entry_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly created Invitation"))
            ),
        )?;

    create_link(
        agent_pub_key.clone(),
        invitation_entry_hash.clone(),
        LinkTypes::AgentToInvites,
        LinkTag::new(String::from("Invitee")),
    )?;

   for agent in invitees_list.0.clone().into_iter() {
        create_link(
            agent,
            invitation_entry_hash.clone(),
            LinkTypes::AgentToInvites,
            LinkTag::new(String::from("Invitee")),
        )?;
    }
    return Ok(record);
}


pub fn get_my_pending_invitations() -> ExternResult<Vec<InvitationEntryInfo>> {
    let agent: AgentPubKey = agent_info()?.agent_latest_pubkey;
    let mut pending_invitations: Vec<InvitationEntryInfo> = vec![];

    let links = get_links(agent, LinkTypes::AgentToInvites, None)?;
    let get_input: Vec<GetInput> = links
        .into_iter()
        .map(|link| GetInput::new(
            EntryHash::from(link.target).into(),
            GetOptions::default(),
        ))
        .collect();
    let records: Vec<Record> = HDK
        .with(|hdk| hdk.borrow().get(get_input))?
        .into_iter()
        .filter_map(|r| r)
        .collect();
    //Ok(records)
    for record in records.into_iter() {
        if let Ok(latest_invite) = get_latest_invite(record.signed_action.action_address().clone()) {
            let invitation_info = get_invitation_entry_info(latest_invite.unwrap());
            pending_invitations.push(invitation_info?); 
        }
    }
    Ok(pending_invitations)
}



/* 

        let invtation_info = record.entry().
        //get(record.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find Invitation: "+hash.clone()))
            ),
        )?;
        pending_invitations.push(record.)
    }


    let agent_pub_key: AgentPubKey = agent_info()?.agent_latest_pubkey;
    let mut pending_invitations: Vec<InvitationEntryInfo> = vec![];

    let pending_invitations_links: Vec<Link> = get_links(
        agent_pub_key,
        LinkTypes::AgentToInvites, None
    )?;

    let invitation_entry_hashes: Vec<EntryHash> = pending_invitations_links
    .into_iter()
    .map(|link|EntryHash::from(link.target).into())
    .collect();

    

    let get_input: Vec<GetInput> = pending_invitations_links
    .into_iter()
    .map(|link| GetInput::new(
        EntryHash::from(link.target).into(),
        GetOptions::default(),
    ))
    .collect();



    //let get_input = pending_invitations_links
      //  .into_iter()
       // .map(|link| GetInput::new(link.target.into(), GetOptions::default()))
       // .collect();

    let get_output = HDK.with(|h| h.borrow().get_details(get_input))?;

    for details in get_output.into_iter().filter_map(|details| details) {
        if let Ok(invitation_info) = get_invitations_entry_info_from_details(details) {
            pending_invitations.push(invitation_info);
        }
    }

    return Ok(pending_invitations);
}
*/

//HELPERS
fn get_invitation_entry_info(invite: Record) -> ExternResult<InvitationEntryInfo> {
    let invite_hash = invite.signed_action.action_address();
    let invitation: Invite = invite.entry.clone().to_app_option().map_err(|e| wasm_error!(e))?.ok_or(
        wasm_error!(
            WasmErrorInner::Guest(String::from("Could not find Invitation for hash "))
        ),
    )?;
//    .ok_or_else(|| {
 //       WasmError::Guest("we dont found the invitation entry for the given hash".into())
 //   })?;
    let invitees_who_accepted: Vec<AgentPubKey> = get_links(
        invite_hash.clone(),
        LinkTypes::InviteToMembers,
        Some(LinkTag::new("Accepted")),
    )?.into_iter()
    .map(|link| AgentPubKey::from(EntryHash::from(link.target)))
    .collect();

    //.into_iter()
    //.map(|link| -> AgentPubKey {
        //let agent_pub_key: AgentPubKey = link.target;
   //     return AgentPubKey::from(link.target).into();
   // })
   // .collect();
    //let details = get_details<ActionHash>(invite_hash.clone(),GetOptions::default())?
    //let invitees_who_rejected: Vec<AgentPubKey> = 
      //  .deletes
       // .into_iter()
      //  .map(|signed_header_hashed| -> AgentPubKeyB64 {
      //      return AgentPubKeyB64::from(signed_header_hashed.header().author().to_owned());
      //  })
      //  .collect();
 
    return Ok(InvitationEntryInfo {
        invitation: invitation.clone(),
        invitation_entry_hash: hash_entry(invitation)?,
        invitation_action_hash: invite_hash.clone(),
        invitees_who_accepted,
        invitees_who_rejected: vec![]
    })
}
/* 

#[hdk_extern]
pub fn create_invite(invite: Invite) -> ExternResult<Record> {
    let invite_hash = create_entry(&EntryTypes::Invite(invite.clone()))?;
    let record = get(invite_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly created Invite"))
            ),
        )?;
    Ok(record)
}
#[hdk_extern]
pub fn get_invite(original_invite_hash: ActionHash) -> ExternResult<Option<Record>> {
    get_latest_invite(original_invite_hash)
}*/
fn get_latest_invite(invite_hash: ActionHash) -> ExternResult<Option<Record>> {
    let details = get_details(invite_hash, GetOptions::default())?
        .ok_or(wasm_error!(WasmErrorInner::Guest("Invite not found".into())))?;
    let record_details = match details {
        Details::Entry(_) => {
            Err(wasm_error!(WasmErrorInner::Guest("Malformed details".into())))
        }
        Details::Record(record_details) => Ok(record_details),
    }?;
    if record_details.deletes.len() > 0 {
        return Ok(None);
    }
    match record_details.updates.last() {
        Some(update) => get_latest_invite(update.action_address().clone()),
        None => Ok(Some(record_details.record)),
    }
}


#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateInviteInput {
    pub previous_invite_hash: ActionHash,
    pub updated_invite: Invite,
}
#[hdk_extern]
pub fn update_invite(input: UpdateInviteInput) -> ExternResult<Record> {
    let updated_invite_hash = update_entry(
        input.previous_invite_hash,
        &input.updated_invite,
    )?;
    let record = get(updated_invite_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly updated Invite"))
            ),
        )?;
    Ok(record)
}
#[hdk_extern]
pub fn delete_invite(original_invite_hash: ActionHash) -> ExternResult<ActionHash> {
    delete_entry(original_invite_hash)
}