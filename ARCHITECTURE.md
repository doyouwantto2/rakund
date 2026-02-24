# Rakund Complete Architecture

## High-Level System Overview

```
┌─────────────────────────────────────────────────────────────────┐
│                        USER INTERFACE                          │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐           │
│  │   Header    │  │   Visual    │  │   Piano     │           │
│  │  Controls   │  │ Display     │  │ Interface   │           │
│  └─────────────┘  └─────────────┘  └─────────────┘           │
└─────────────────────────────────────────────────────────────────┘
                              │
                    ┌─────────▼─────────┐
                    │   APPLICATION     │
                    │   STATE LAYER     │
                    │  (Hooks & Stores)  │
                    └─────────┬─────────┘
                              │
                    ┌─────────▼─────────┐
                    │   COMMUNICATION   │
                    │      BRIDGE      │
                    │   (Tauri IPC)     │
                    └─────────┬─────────┘
                              │
┌─────────────────────────────▼───────────────────────────────────┐
│                      BACKEND SYSTEM                            │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐           │
│  │    AUDIO    │  │   DATA      │  │   SYSTEM    │           │
│  │   ENGINE    │  │  MANAGER    │  │  SERVICES   │           │
│  └─────────────┘  └─────────────┘  └─────────────┘           │
└─────────────────────────────────────────────────────────────────┘
                              │
                    ┌─────────▼─────────┐
                    │   HARDWARE/OS    │
                    │   AUDIO OUTPUT   │
                    └───────────────────┘
```

## Frontend Architecture - UI Layer

### Component Structure
```
Main Application Window
├── Header Section (Control Center)
│   ├── Instrument Management
│   │   ├── Available instruments dropdown
│   │   ├── Loading progress indicator
│   │   └── Current instrument display
│   ├── Hand Controls (Octave & Modifiers)
│   │   ├── Left hand: Octave (B/G keys), Layer, Modifiers (♯/♭)
│   │   └── Right hand: Octave (N/H keys), Layer, Modifiers (♯/♭)
│   └── Session Controls
│       ├── Song selection & loading
│       ├── Play/Pause/Stop controls
│       └── Mode selection (Perform/Instruct)
│
├── Visualization Area (Dynamic Content)
│   ├── Rain Layout (MIDI waterfall display)
│   │   ├── Falling notes with timing
│   │   ├── Hand color coding (blue/green)
│   │   └── Real-time position tracking
│   ├── Sheet Layout (Traditional notation)
│   │   ├── Musical staff display
│   │   ├── Note symbols and timing
│   │   └── Key signature display
│   └── Config Page (Settings & preferences)
│       ├── Audio settings
│       ├── Learning options
│       └── Application preferences
│
└── Piano Interface (88-key interactive piano)
    ├── White keys (52 keys) with octave labels
    ├── Black keys (36 keys) with positioning
    ├── Hand highlighting (blue/green borders)
    └── Active note indication (emerald highlighting)
```

### State Management Layer
```
Application State (Hooks)
├── usePiano (Instrument & Playing State)
│   ├── Current instrument & layers
│   ├── Hand octave positions
│   ├── Active notes tracking
│   ├── Keyboard input handling
│   └── Audio note triggers
│
└── useBuffer (Session & Playback State)
    ├── Available songs list
    ├── Current session data
    ├── Playback timing & controls
    ├── Mode selection (perform/instruct)
    └── Real-time position tracking
```

### User Interaction Patterns
```
Piano Playing Flow:
1. User presses keyboard key (z,x,c... or m,j,k...)
2. UI highlights corresponding piano key
3. State updates with active note
4. Audio backend plays sound
5. Visual feedback continues until key release

Song Learning Flow:
1. User selects song from header dropdown
2. System loads MIDI data and shows visualization
3. User chooses learning mode (perform/instruct)
4. Playback starts with visual guidance
5. Piano highlights show which keys to play
6. User plays along with real-time feedback

Instrument Management Flow:
1. User browses available instruments
2. System loads selected instrument with progress
3. Piano interface updates with new sounds
4. Layer/velocity options adjust automatically
5. Last instrument saved for next session
```

## Backend Architecture - System Layer

### Core Service Structure
```
Backend System
├── Audio Engine (Real-time Sound Processing)
│   ├── Voice Management
│   │   ├── Active voice tracking
│   │   ├── Note on/off handling
│   │   └── Volume envelope control
│   ├── Sample Processing
│   │   ├── Audio sample caching
│   │   ├── Pitch shifting for transposition
│   │   └── Sample interpolation
│   └── Output Stream
│       ├── CPAL audio stream management
│       ├── Real-time audio rendering
│       └── System audio output
│
├── Data Manager (File & State Handling)
│   ├── Instrument Management
│   │   ├── JSON configuration parsing
│   │   ├── Audio file loading (FLAC/WAV)
│   │   └── Sample caching system
│   ├── MIDI Processing
│   │   ├── MIDI file parsing
│   │   ├── Timing conversion (ticks → ms)
│   │   └── Note data extraction
│   └── State Persistence
│       ├── Last instrument memory
│       ├── User preferences
│       └── Application settings
│
└── System Services (Platform Integration)
    ├── File System Access
    │   ├── Instruments directory scanning
    │   ├── Songs directory management
    │   └── Configuration file handling
    ├── Tauri Command Interface
    │   ├── IPC command registration
    │   ├── Frontend-backend communication
    │   └── Error handling & responses
    └── Resource Management
        ├── Memory optimization
        ├── Cache cleanup
        └── Performance monitoring
```

### Audio Processing Pipeline
```
Note Trigger Pipeline:
1. Frontend: User presses key → MIDI note generation
2. IPC Bridge: invoke("play_midi_note") with parameters
3. Backend: Receive command with MIDI, velocity, layer
4. Audio Engine:
   ├── Lookup instrument configuration
   ├── Select appropriate sample layer
   ├── Calculate pitch ratio for transposition
   ├── Create voice with sample data
   └── Add to active voices list
5. Real-time Renderer:
   ├── Process all active voices
   ├── Apply pitch shifting and volume
   ├── Mix voices together
   ├── Send to system audio output
   └── Remove finished voices

Audio Output Flow:
├── Sample Cache: In-memory audio data storage
├── Voice Pool: Active note tracking system
├── Mixer: Combine multiple voices with gain control
├── Effects: Volume envelopes, release handling
└── System Audio: CPAL stream to OS audio subsystem
```

### Data Management System
```
Instrument Loading:
1. Scan instruments directory for folders with instrument.json
2. Parse JSON configuration with layer mappings
3. Load audio samples (FLAC/WAV) into memory cache
4. Validate sample data and pitch information
5. Store in global instrument state
6. Report loading progress to frontend

MIDI Session Management:
1. Parse MIDI file from songs directory
2. Convert timing from ticks to milliseconds
3. Extract note data (MIDI, velocity, timing)
4. Store in session buffer for visualization
5. Provide note list to frontend for display
6. Handle session cleanup and memory management

State Persistence:
1. Application startup: Read last instrument from storage
2. Instrument changes: Save current selection
3. User preferences: Store settings and configurations
4. Error recovery: Maintain consistent state
```

## Communication Bridge - Frontend/Backend Integration

### IPC Command Structure
```
Frontend → Backend Commands:
├── Audio Commands
│   ├── play_midi_note(midi, velocity, layer)
│   ├── stop_midi_note(midi)
│   └── set_sustain(active)
├── Instrument Commands
│   ├── get_available_instruments()
│   ├── load_instrument(folder)
│   ├── get_instrument_info()
│   └── clear_last_instrument()
├── Session Commands
│   ├── scan_songs()
│   ├── load_midi_session(filePath)
│   ├── get_session_notes()
│   └── clear_session()
└── System Commands
    └── get_app_state()

Backend → Frontend Events:
├── Progress Updates
│   ├── load_progress(progress, status, message)
│   └── Instrument loading feedback
└── State Changes
    └── Application state synchronization
```

### Data Flow Patterns
```
User Action Flow:
User Input → Frontend Component → State Hook → IPC Command → Backend Service → System Response → UI Update

Example: Piano Key Press
1. User presses 'z' key
2. PianoContainer detects keyboard event
3. usePiano hook converts to MIDI note 36
4. invoke("play_midi_note", {midi:36, velocity:64, layer:"MP"})
5. Backend receives command, processes audio
6. Frontend updates visual state (key highlight)
7. Audio plays through system speakers

Example: Song Loading
1. User selects song from dropdown
2. HeaderContainer triggers loadSession()
3. useBuffer hook calls invoke("load_midi_session")
4. Backend parses MIDI file, stores in buffer
5. Frontend receives session info
6. Visualization updates with note data
7. Playback controls become active
```

## System Integration & Performance

### Memory Management
```
Frontend Memory:
├── Reactive State: SolidJS signals for efficient updates
├── Component Lifecycle: Automatic cleanup on unmount
├── Event Listeners: Proper removal to prevent leaks
└── Data Caching: Session data stored in hooks

Backend Memory:
├── Sample Cache: In-memory audio data with LRU cleanup
├── Voice Pool: Automatic removal of finished voices
├── Session Buffer: Clear on song change/unload
└── Arc/Mutex: Thread-safe shared state management
```

### Performance Optimizations
```
Frontend Optimizations:
├── SolidJS Reactivity: Fine-grained updates, no virtual DOM
├── RequestAnimationFrame: Smooth animation timing
├── Memoized Calculations: Piano positioning, highlights
├── Event Delegation: Efficient keyboard handling
└── Lazy Loading: Components load as needed

Backend Optimizations:
├── Real-time Audio: Low-latency CPAL stream processing
├── Sample Caching: Pre-load frequently used samples
├── Voice Pooling: Reuse voice objects
├── Linear Interpolation: Fast sample playback
├── Arc Sharing: Efficient memory sharing
└── Async Processing: Non-blocking file operations
```

### Error Handling & Recovery
```
Frontend Error Handling:
├── User Input Validation: Prevent invalid commands
├── Network Error Recovery: Retry failed IPC calls
├── State Reset: Clear corrupted sessions
└── User Feedback: Clear error messages

Backend Error Handling:
├── Audio Device Recovery: Handle device disconnection
├── File I/O Errors: Graceful handling of missing files
├── Memory Allocation: Fallback for allocation failures
├── Parsing Errors: Invalid JSON/audio format handling
└── System Integration: OS-specific error handling
```

## Security & Safety

### Frontend Security
```
Input Validation:
├── Keyboard Input: Validate key mappings
├── File Paths: Prevent directory traversal
├── User Data: Sanitize all inputs
└── State Validation: Ensure consistent state

Code Safety:
├── No eval(): Safe frontend execution
├── Type Safety: TypeScript validation
├── Component Isolation: Prevent side effects
└── Memory Safety: Automatic cleanup
```

### Backend Security
```
System Access:
├── Tauri Capabilities: Restricted file system access
├── Path Validation: Prevent directory traversal attacks
├── Resource Limits: Prevent resource exhaustion
└── Sandboxing: Limited system interaction

Data Safety:
├── Memory Safety: Rust's ownership system
├── Thread Safety: Arc/Mutex for shared state
├── Error Propagation: Result types for error handling
└── Resource Cleanup: Drop traits for automatic cleanup
```

## Extension Points & Future Development

### Frontend Extensions
```
Visualization Modes:
├── Current: Rain layout, sheet music
├── Potential: Keyboard view, frequency analysis
├── Framework: SceneDispatcher pattern for easy addition
└── Data Source: Hook-based state management

Learning Features:
├── Current: Perform/Instruct modes
├── Future: Scoring system, progress tracking
├── Adaptive: Difficulty adjustment based on performance
└── Social: Multiplayer sessions, sharing
```

### Backend Extensions
```
Audio Features:
├── Current: Sample-based synthesis
├── Future: Physical modeling, effects processing
├── Formats: Additional audio format support
└── Export: Recording, MIDI file generation

Instrument Support:
├── Current: JSON + FLAC/WAV configuration
├── Future: SF2, VSTi support
├── Streaming: Large sample set handling
└── Custom: User-defined instruments
```

This architecture provides a comprehensive overview of the entire system, from user interface through application logic to system integration, emphasizing the high-level connections and data flows rather than implementation details.
