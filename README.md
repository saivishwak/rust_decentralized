# BitFabRust
Building a peer to peer decentralized compute platform to run high demanding tasks distributedly.

## To Do
- [ ] Add peer discovery as current approach is doing uneccessary broadcast
- [x] **IMP** - Need to check why after creating 4 peers the 4th peer stucks in process server info awaiting lock
- [ ] Architect channels and different components properly
- [x] Proper Error Handling
- [x] Add channels so that inter thread communication is possible (ex, http and p2p)
- [x] check if the processNewPeer handler a blocking task as we are awaiting there
- [ ] refactor the entire code
- [ ] make stream_id as UUID
- [x] make stream_id unique so that boradcast_to_peers can work
- [x] Move router to sever struct
- [x] Implment the handle_connection in server struct
- [x] Implement the connect_to_peer in server struct

The reason to do them is that we need to access them from the router closures
