use crate::{requests, locate, errors::RiftInitializationError};

// Everything you'll need to use to query the League API can be found within the LeagueClient.
pub struct LeagueClient {
    pub lcu: requests::lcu::RequestClient,
    pub live: requests::live_client_data::LiveClientData
}

// Gets a LeagueClient, you should make sure that League of Legends is active on the computer before calling this method. 
// If League of Legends exits during your applications lifetime, you should to get a new LeagueClient, since each LeagueClient is unique to its assigned proccess. This is however only worth caring about if you're using the League Client API features.
pub fn get_league_client() -> Result<LeagueClient, RiftInitializationError> {
    let lockfile = locate::lock_file::get_lockfile();
    match lockfile {
        Ok(l) => Ok(LeagueClient {live: requests::live_client_data::LiveClientData::new(), lcu: requests::lcu::get_request_client(l)}),
        Err(e) => return Err(e)
    }
}