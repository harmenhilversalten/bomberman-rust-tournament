### Events Crate (`bomberman_events`)

#### Module Structure
```
bomberman_events/
├── lib.rs                 // Crate public interface
├── events/                // Event definitions
│   ├── mod.rs
│   ├── game_events.rs     // Game event definitions
│   ├── bot_events.rs      // Bot-specific events
│   └── system_events.rs   // System internal events
├── bus/                   // Event bus implementation
│   ├── mod.rs
│   ├── event_bus.rs       // Main event bus
│   ├── subscriber.rs      // Event subscribers
│   └── filter.rs          // Event filtering
├── queue/                 // Event queue management
│   ├── mod.rs
│   ├── priority_queue.rs  // Priority event queue
│   └── batch_queue.rs     // Batched event processing
├── serialization/         // Event serialization
│   ├── mod.rs
│   ├── encoder.rs         // Event encoding
│   └── decoder.rs         // Event decoding
└── tests/                 // Unit and integration tests
    ├── mod.rs
    ├── event_tests.rs
    └── bus_tests.rs
```

#### Core Data Structures

```rust
// Event bus for efficient event handling
pub struct EventBus {
    queues: HashMap<EventType, EventQueue>,
    subscribers: HashMap<EventType, Vec<SubscriberId>>,
    subscriber_info: HashMap<SubscriberId, SubscriberInfo>,
    next_subscriber_id: AtomicU32,
    stats: EventBusStats,
}

// Event queue with priority support
pub struct EventQueue {
    high_priority: SegQueue<GameEvent>,
    normal_priority: SegQueue<GameEvent>,
    low_priority: SegQueue<GameEvent>,
    pending_count: AtomicUsize,
}

// Event subscriber information
pub struct SubscriberInfo {
    id: SubscriberId,
    name: String,
    filter: Option<EventFilter>,
    channel: Sender<GameEvent>,
    active: AtomicBool,
}

// Game event definitions
#[derive(Debug, Clone, PartialEq)]
pub enum GameEvent {
    // Entity events
    EntityMoved {
        entity_id: EntityId,
        old_position: Position,
        new_position: Position,
    },
    EntityDestroyed {
        entity_id: EntityId,
    },
    
    // Bomb events
    BombPlaced {
        entity_id: EntityId,
        bomb_id: BombId,
        position: Position,
        power: u8,
    },
    BombExploded {
        bomb_id: BombId,
        position: Position,
        power: u8,
    },
    
    // Player events
    PlayerDamaged {
        entity_id: EntityId,
        damage: u8,
        source: DamageSource,
    },
    PlayerDied {
        entity_id: EntityId,
    },
    
    // Game state events
    TickCompleted {
        tick_number: u64,
        duration: Duration,
    },
    GameStarted {
        seed: u64,
        player_count: u8,
    },
    GameEnded {
        winner: Option<EntityId>,
        reason: GameEndReason,
    },
    
    // Bot events
    BotDecision {
        bot_id: BotId,
        decision: BotDecision,
        decision_time: Duration,
    },
    BotError {
        bot_id: BotId,
        error: BotError,
    },
}

// Event filter for selective subscription
pub struct EventFilter {
    include_types: HashSet<EventType>,
    exclude_types: HashSet<EventType>,
    entity_filter: Option<EntityFilter>,
    custom_filter: Option<Box<dyn Fn(&GameEvent) -> bool + Send + Sync>>,
}
```

#### Key Algorithms

1. **Event Processing**
   - Priority-based event processing
   - Batched event delivery for performance
   - Filtering and routing optimization

```rust
impl EventBus {
    pub fn process_events(&mut self) -> Result<usize, EventError> {
        let mut processed_count = 0;
        
        // Process high priority events first
        for (_, queue) in &mut self.queues {
            processed_count += self.process_queue_by_priority(queue, EventPriority::High)?;
        }
        
        // Process normal priority events
        for (_, queue) in &mut self.queues {
            processed_count += self.process_queue_by_priority(queue, EventPriority::Normal)?;
        }
        
        // Process low priority events
        for (_, queue) in &mut self.queues {
            processed_count += self.process_queue_by_priority(queue, EventPriority::Low)?;
        }
        
        // Update statistics
        self.stats.processed_events.fetch_add(processed_count, Ordering::Relaxed);
        
        Ok(processed_count)
    }
    
    fn process_queue_by_priority(
        &mut self,
        queue: &mut EventQueue,
        priority: EventPriority,
    ) -> Result<usize, EventError> {
        let mut processed_count = 0;
        let mut batch = Vec::with_capacity(EVENT_BATCH_SIZE);
        
        // Collect events of the specified priority
        let event_queue = match priority {
            EventPriority::High => &queue.high_priority,
            EventPriority::Normal => &queue.normal_priority,
            EventPriority::Low => &queue.low_priority,
        };
        
        // Collect a batch of events
        while let Ok(event) = event_queue.pop() {
            batch.push(event);
            processed_count += 1;
            
            if batch.len() >= EVENT_BATCH_SIZE {
                break;
            }
        }
        
        // Update pending count
        queue.pending_count.fetch_sub(batch.len(), Ordering::Relaxed);
        
        // Process the batch
        if !batch.is_empty() {
            self.process_event_batch(batch, priority)?;
        }
        
        Ok(processed_count)
    }
    
    fn process_event_batch(
        &mut self,
        events: Vec<GameEvent>,
        priority: EventPriority,
    ) -> Result<(), EventError> {
        // Group events by type for efficient delivery
        let mut events_by_type: HashMap<EventType, Vec<GameEvent>> = HashMap::new();
        
        for event in events {
            let event_type = EventType::from(&event);
            events_by_type.entry(event_type).or_insert_with(Vec::new).push(event);
        }
        
        // Deliver events to subscribers
        for (event_type, event_list) in events_by_type {
            if let Some(subscribers) = self.subscribers.get(&event_type) {
                for subscriber_id in subscribers {
                    if let Some(subscriber_info) = self.subscriber_info.get(subscriber_id) {
                        if subscriber_info.active.load(Ordering::Relaxed) {
                            self.deliver_events_to_subscriber(
                                subscriber_id,
                                subscriber_info,
                                &event_list,
                                priority,
                            )?;
                        }
                    }
                }
            }
        }
        
        Ok(())
    }
    
    fn deliver_events_to_subscriber(
        &mut self,
        subscriber_id: SubscriberId,
        subscriber_info: &SubscriberInfo,
        events: &[GameEvent],
        priority: EventPriority,
    ) -> Result<(), EventError> {
        // Filter events based on subscriber's filter
        let filtered_events = if let Some(ref filter) = subscriber_info.filter {
            events.iter()
                .filter(|event| filter.matches(event))
                .cloned()
                .collect()
        } else {
            events.to_vec()
        };
        
        // Skip if no events after filtering
        if filtered_events.is_empty() {
            return Ok(());
        }
        
        // Send events to subscriber
        for event in filtered_events {
            // Check if subscriber is still active
            if !subscriber_info.active.load(Ordering::Relaxed) {
                return Err(EventError::SubscriberNotActive(subscriber_id));
            }
            
            // Try to send event, with timeout
            match subscriber_info.channel.send_timeout(
                event,
                Duration::from_millis(EVENT_SEND_TIMEOUT_MS),
            ) {
                Ok(()) => {
                    // Update statistics
                    self.stats.delivered_events.fetch_add(1, Ordering::Relaxed);
                }
                Err(SendTimeoutError::Timeout(_)) => {
                    // Subscriber is too slow, deactivate it
                    subscriber_info.active.store(false, Ordering::Relaxed);
                    return Err(EventError::SubscriberTimeout(subscriber_id));
                }
                Err(SendTimeoutError::Disconnected(_)) => {
                    // Subscriber is disconnected, deactivate it
                    subscriber_info.active.store(false, Ordering::Relaxed);
                    return Err(EventError::SubscriberDisconnected(subscriber_id));
                }
            }
        }
        
        Ok(())
    }
}
```

2. **Event Filtering**
   - Efficient event filtering with minimal overhead
   - Type-based and custom filtering support
   - Entity-based filtering for spatial events

```rust
impl EventFilter {
    pub fn matches(&self, event: &GameEvent) -> bool {
        // Check type inclusion/exclusion
        let event_type = EventType::from(event);
        
        if !self.include_types.is_empty() && !self.include_types.contains(&event_type) {
            return false;
        }
        
        if self.exclude_types.contains(&event_type) {
            return false;
        }
        
        // Check entity filter
        if let Some(ref entity_filter) = self.entity_filter {
            if !entity_filter.matches(event) {
                return false;
            }
        }
        
        // Check custom filter
        if let Some(ref custom_filter) = self.custom_filter {
            if !custom_filter(event) {
                return false;
            }
        }
        
        true
    }
}

impl EntityFilter {
    pub fn matches(&self, event: &GameEvent) -> bool {
        match *event {
            GameEvent::EntityMoved { entity_id, .. } => self.matches_entity(entity_id),
            GameEvent::EntityDestroyed { entity_id } => self.matches_entity(entity_id),
            GameEvent::BombPlaced { entity_id, .. } => self.matches_entity(entity_id),
            GameEvent::PlayerDamaged { entity_id, .. } => self.matches_entity(entity_id),
            GameEvent::PlayerDied { entity_id } => self.matches_entity(entity_id),
            GameEvent::BotDecision { bot_id, .. } => self.matches_bot(bot_id),
            GameEvent::BotError { bot_id, .. } => self.matches_bot(bot_id),
            _ => true, // Non-entity events always pass
        }
    }
    
    fn matches_entity(&self, entity_id: EntityId) -> bool {
        match self {
            EntityFilter::Specific(id) => *id == entity_id,
            EntityFilter::Bot(bot_id) => {
                // This would require access to the game state to check entity-bot mapping
                // In a real implementation, we'd need a way to access this information
                true
            }
            EntityFilter::InArea { center, radius } => {
                // This would require access to entity positions
                // In a real implementation, we'd need a way to access this information
                true
            }
            EntityFilter::Custom(filter) => filter(entity_id),
        }
    }
    
    fn matches_bot(&self, bot_id: BotId) -> bool {
        match self {
            EntityFilter::Bot(id) => *id == bot_id,
            _ => true,
        }
    }
}
```

#### Performance Optimizations

1. **Lock-Free Queues**: Use lock-free SegQueue for concurrent event handling.

2. **Priority-Based Processing**: Process high-priority events first for critical path optimization.

3. **Batched Delivery**: Deliver events in batches to reduce synchronization overhead.

4. **Zero-Copy Filtering**: Filter events without copying when possible.

5. **Subscriber Timeouts**: Detect and deactivate slow subscribers to prevent system stalls.

#### API Design

```rust
// Main public interface for the events crate
pub struct EventBusBuilder {
    queue_capacity: usize,
    max_subscribers: usize,
    stats_enabled: bool,
}

impl EventBusBuilder {
    pub fn new() -> Self { /* ... */ }
    
    pub fn queue_capacity(mut self, capacity: usize) -> Self { /* ... */ }
    
    pub fn max_subscribers(mut self, max: usize) -> Self { /* ... */ }
    
    pub fn enable_stats(mut self, enabled: bool) -> Self { /* ... */ }
    
    pub fn build(self) -> EventBus { /* ... */ }
}

impl EventBus {
    /// Create a new event bus with default settings
    pub fn new() -> Self {
        EventBusBuilder::new().build()
    }
    
    /// Emit an event to the bus
    pub fn emit(&mut self, event: GameEvent, priority: EventPriority) -> Result<(), EventError> {
        let event_type = EventType::from(&event);
        
        // Get or create queue for this event type
        let queue = self.queues.entry(event_type).or_insert_with(|| EventQueue::new());
        
        // Add event to appropriate priority queue
        match priority {
            EventPriority::High => queue.high_priority.push(event),
            EventPriority::Normal => queue.normal_priority.push(event),
            EventPriority::Low => queue.low_priority.push(event),
        }
        
        // Update pending count
        queue.pending_count.fetch_add(1, Ordering::Relaxed);
        
        // Update statistics
        self.stats.emitted_events.fetch_add(1, Ordering::Relaxed);
        
        Ok(())
    }
    
    /// Subscribe to events of a specific type
    pub fn subscribe<F>(
        &mut self,
        event_type: EventType,
        handler: F,
    ) -> Result<SubscriptionHandle, EventError>
    where
        F: Fn(GameEvent) + Send + 'static,
    {
        self.subscribe_with_filter(event_type, handler, None)
    }
    
    /// Subscribe to events with a custom filter
    pub fn subscribe_with_filter<F>(
        &mut self,
        event_type: EventType,
        handler: F,
        filter: Option<EventFilter>,
    ) -> Result<SubscriptionHandle, EventError>
    where
        F: Fn(GameEvent) + Send + 'static,
    {
        // Check subscriber limit
        if self.subscriber_info.len() >= self.max_subscribers {
            return Err(EventError::TooManySubscribers);
        }
        
        // Create channel for event delivery
        let (sender, receiver) = bounded(EVENT_CHANNEL_CAPACITY);
        
        // Generate subscriber ID
        let subscriber_id = SubscriberId::new(
            self.next_subscriber_id.fetch_add(1, Ordering::Relaxed),
        );
        
        // Create subscriber info
        let subscriber_info = SubscriberInfo {
            id: subscriber_id,
            name: format!("Subscriber-{}", subscriber_id),
            filter,
            channel: sender,
            active: AtomicBool::new(true),
        };
        
        // Store subscriber info
        self.subscriber_info.insert(subscriber_id, subscriber_info);
        
        // Add to subscriber list for this event type
        self.subscribers.entry(event_type).or_insert_with(Vec::new).push(subscriber_id);
        
        // Spawn handler task
        let handle = SubscriptionHandle::new(subscriber_id);
        let handle_clone = handle.clone();
        
        tokio::spawn(async move {
            Self::event_handler_task(receiver, handler, handle_clone).await;
        });
        
        Ok(handle)
    }
    
    /// Process all pending events
    pub fn process_events(&mut self) -> Result<usize, EventError> { /* ... */ }
    
    /// Get event bus statistics
    pub fn stats(&self) -> &EventBusStats {
        &self.stats
    }
    
    /// Unsubscribe from events
    pub fn unsubscribe(&mut self, handle: SubscriptionHandle) -> Result<(), EventError> {
        let subscriber_id = handle.id();
        
        // Remove subscriber info
        if let Some(subscriber_info) = self.subscriber_info.get(&subscriber_id) {
            // Mark as inactive
            subscriber_info.active.store(false, Ordering::Relaxed);
            
            // Remove from all event type subscriber lists
            for (_, subscribers) in &mut self.subscribers {
                subscribers.retain(|&id| id != subscriber_id);
            }
            
            // Remove from subscriber info
            self.subscriber_info.remove(&subscriber_id);
            
            Ok(())
        } else {
            Err(EventError::SubscriberNotFound(subscriber_id))
        }
    }
    
    async fn event_handler_task<F>(
        receiver: Receiver<GameEvent>,
        handler: F,
        handle: SubscriptionHandle,
    ) where
        F: Fn(GameEvent) + Send + 'static,
    {
        while !handle.is_cancelled() {
            match receiver.recv().await {
                Ok(event) => {
                    handler(event);
                }
                Err(RecvError::Closed) => {
                    // Channel closed, exit task
                    break;
                }
                Err(RecvError::Timeout) => {
                    // Timeout, continue waiting
                    continue;
                }
            }
        }
    }
}

/// Handle for an event subscription
#[derive(Debug, Clone)]
pub struct SubscriptionHandle {
    id: SubscriberId,
    cancelled: Arc<AtomicBool>,
}

impl SubscriptionHandle {
    fn new(id: SubscriberId) -> Self {
        Self {
            id,
            cancelled: Arc::new(AtomicBool::new(false)),
        }
    }
    
    fn id(&self) -> SubscriberId {
        self.id
    }
    
    fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::Relaxed)
    }
    
    /// Cancel the subscription
    pub fn cancel(&self) {
        self.cancelled.store(true, Ordering::Relaxed);
    }
}

/// Event priority levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum EventPriority {
    Low = 0,
    Normal = 1,
    High = 2,
}

/// Event bus statistics
#[derive(Debug, Clone)]
pub struct EventBusStats {
    emitted_events: AtomicUsize,
    processed_events: AtomicUsize,
    delivered_events: AtomicUsize,
    dropped_events: AtomicUsize,
    active_subscribers: AtomicUsize,
}
```

#### Error Handling Strategy

```rust
/// Errors that can occur in event operations
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EventError {
    /// Too many subscribers
    TooManySubscribers,
    /// Subscriber not found
    SubscriberNotFound(SubscriberId),
    /// Subscriber not active
    SubscriberNotActive(SubscriberId),
    /// Subscriber timeout
    SubscriberTimeout(SubscriberId),
    /// Subscriber disconnected
    SubscriberDisconnected(SubscriberId),
    /// Event queue full
    QueueFull(EventType),
    /// Invalid event
    InvalidEvent(String),
    /// Serialization error
    Serialization(SerializationError),
    /// I/O error
    Io(String),
}

impl fmt::Display for EventError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EventError::TooManySubscribers => write!(f, "Too many subscribers"),
            EventError::SubscriberNotFound(id) => write!(f, "Subscriber not found: {:?}", id),
            EventError::SubscriberNotActive(id) => write!(f, "Subscriber not active: {:?}", id),
            EventError::SubscriberTimeout(id) => write!(f, "Subscriber timeout: {:?}", id),
            EventError::SubscriberDisconnected(id) => write!(f, "Subscriber disconnected: {:?}", id),
            EventError::QueueFull(event_type) => write!(f, "Event queue full for type: {:?}", event_type),
            EventError::InvalidEvent(msg) => write!(f, "Invalid event: {}", msg),
            EventError::Serialization(err) => write!(f, "Serialization error: {}", err),
            EventError::Io(msg) => write!(f, "I/O error: {}", msg),
        }
    }
}

impl std::error::Error for EventError {}

impl From<SerializationError> for EventError {
    fn from(err: SerializationError) -> Self {
        EventError::Serialization(err)
    }
}

impl From<std::io::Error> for EventError {
    fn from(err: std::io::Error) -> Self {
        EventError::Io(err.to_string())
    }
}
```

#### Testing Strategy

1. **Unit Tests**
   - Test event emission and processing
   - Verify subscription management
   - Test event filtering

2. **Integration Tests**
   - Test event bus with mock subscribers
   - Verify priority-based processing
   - Test concurrent event handling

3. **Property-Based Tests**
   - Generate random event sequences
   - Verify event delivery guarantees
   - Test filtering correctness

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;
    use std::time::Duration;
    
    #[test]
    fn test_event_emission_and_processing() {
        let mut bus = EventBus::new();
        let events_received = Arc::new(Mutex::new(Vec::new()));
        let events_received_clone = events_received.clone();
        
        // Subscribe to entity moved events
        bus.subscribe(EventType::EntityMoved, move |event| {
            if let GameEvent::EntityMoved { .. } = event {
                events_received_clone.lock().unwrap().push(event);
            }
        }).unwrap();
        
        // Emit an event
        bus.emit(
            GameEvent::EntityMoved {
                entity_id: EntityId::new(1),
                old_position: Position { x: 0, y: 0, subpixel_x: 0, subpixel_y: 0 },
                new_position: Position { x: 1, y: 0, subpixel_x: 0, subpixel_y: 0 },
            },
            EventPriority::Normal,
        ).unwrap();
        
        // Process events
        let processed = bus.process_events().unwrap();
        assert_eq!(processed, 1);
        
        // Verify event was received
        let received = events_received.lock().unwrap();
        assert_eq!(received.len(), 1);
    }
    
    #[test]
    fn test_event_filtering() {
        let mut bus = EventBus::new();
        let events_received = Arc::new(Mutex::new(Vec::new()));
        let events_received_clone = events_received.clone();
        
        // Create a filter for specific entity
        let filter = EventFilter::new()
            .with_entity_filter(EntityFilter::Specific(EntityId::new(1)));
        
        // Subscribe with filter
        bus.subscribe_with_filter(
            EventType::EntityMoved,
            move |event| {
                events_received_clone.lock().unwrap().push(event);
            },
            Some(filter),
        ).unwrap();
        
        // Emit events for different entities
        bus.emit(
            GameEvent::EntityMoved {
                entity_id: EntityId::new(1),
                old_position: Position { x: 0, y: 0, subpixel_x: 0, subpixel_y: 0 },
                new_position: Position { x: 1, y: 0, subpixel_x: 0, subpixel_y: 0 },
            },
            EventPriority::Normal,
        ).unwrap();
        
        bus.emit(
            GameEvent::EntityMoved {
                entity_id: EntityId::new(2),
                old_position: Position { x: 0, y: 0, subpixel_x: 0, subpixel_y: 0 },
                new_position: Position { x: 1, y: 0, subpixel_x: 0, subpixel_y: 0 },
            },
            EventPriority::Normal,
        ).unwrap();
        
        // Process events
        bus.process_events().unwrap();
        
        // Verify only filtered event was received
        let received = events_received.lock().unwrap();
        assert_eq!(received.len(), 1);
        
        if let GameEvent::EntityMoved { entity_id, .. } = received[0] {
            assert_eq!(entity_id, EntityId::new(1));
        } else {
            panic!("Unexpected event type");
        }
    }
    
    proptest! {
        #[test]
        fn test_concurrent_event_processing(
            event_count in 1usize..1000,
            subscriber_count in 1usize..10,
        ) {
            let mut bus = EventBus::new();
            let counters = Arc::new(Mutex::new(vec![0usize; subscriber_count]));
            
            // Create subscribers
            for i in 0..subscriber_count {
                let counters_clone = counters.clone();
                bus.subscribe(EventType::EntityMoved, move |_event| {
                    let mut counters = counters_clone.lock().unwrap();
                    counters[i] += 1;
                }).unwrap();
            }
            
            // Emit events
            for _ in 0..event_count {
                bus.emit(
                    GameEvent::EntityMoved {
                        entity_id: EntityId::new(1),
                        old_position: Position { x: 0, y: 0, subpixel_x: 0, subpixel_y: 0 },
                        new_position: Position { x: 1, y: 0, subpixel_x: 0, subpixel_y: 0 },
                    },
                    EventPriority::Normal,
                ).unwrap();
            }
            
            // Process events
            let processed = bus.process_events().unwrap();
            assert_eq!(processed, event_count);
            
            // Verify all subscribers received all events
            let counters = counters.lock().unwrap();
            for i in 0..subscriber_count {
                assert_eq!(counters[i], event_count);
            }
        }
    }
}
```

