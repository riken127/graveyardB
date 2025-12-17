use tonic::{Streaming, async_trait};

use crate::{api::{AppendEventRequest, AppendEventResponse, GetEventsRequest, event_store_server::EventStore}, domain::event::Event, GraveyardB};


// #[async_trait]
// impl EventStore for GraveyardB {
//     async fn append_event(
//         &self,
//         request: tonic::Request<AppendEventRequest>,
//     ) -> Result<tonic::Response<AppendEventResponse>, tonic::Status> {
//         let req = request.into_inner();

//         print!("Received AppendEventRequest: {:?}", req);

//         for e in req.events {
//             print!("Appending event: {:?}", e);
//         }

//         Ok(tonic::Response::new(
//             AppendEventResponse { success: true }
//         ))
//     }

//     async fn get_events(
//         &self,
//         _request: tonic::Request<GetEventsRequest>,
//     ) -> Result<tonic::Response<Self::GetEventsStream>, tonic::Status> {
//         unimplemented!()
//     }
// }