enum UserEvent {
    BuildThing {
        process: Process
    }
}

enum MainEvent {
    UserEvent {
        user: User,
        event: UserEvent
    }
}

struct Event {
    id: u64,
    event: EventType
};


fn process_event(s: State, e: Event) -> State {

}

