# Frontend Structure - Modular Architecture

## 📁 Directory Structure

```
src/
├── components/
│   ├── whitekey.tsx      # Individual white piano key
│   ├── blackkey.tsx      # Individual black piano key  
│   ├── velocity.tsx      # Velocity layer selector
│   ├── volume.tsx        # Volume/resonance indicator
│   ├── piano.tsx         # Complete piano component
│   └── header.tsx        # Header with title and controls
├── hooks/
│   └── usePiano.ts       # Piano logic and state management
├── pages/
│   ├── home/
│   │   └── home.tsx      # Main page (now simplified)
│   └── config/
│       └── config.tsx    # Settings page (empty for future)
├── assets/
│   └── styles/
│       └── global.css    # Global styles
├── index.tsx             # App entry point
└── vite-env.d.ts         # Vite types
```

## 🧩 Component Breakdown

### 1. **WhiteKey Component** (`whitekey.tsx`)
- **Purpose**: Renders individual white piano keys
- **Props**: `midi`, `active`, `onMouseDown`, `onMouseUp`, `onMouseLeave`
- **Features**: Note labels, active state styling, hover effects

### 2. **BlackKey Component** (`blackkey.tsx`)  
- **Purpose**: Renders individual black piano keys
- **Props**: `active`, `onMouseDown`, `onMouseUp`, `onMouseLeave`, `style`
- **Features**: Positioning, active state styling, hover effects

### 3. **VelocitySelector Component** (`velocity.tsx`)
- **Purpose**: Layer selection buttons (PP, MP, MF, FF)
- **Props**: `selectedLayer`, `onLayerChange`, `layers`
- **Features**: Active layer highlighting, hover effects

### 4. **VolumeControl Component** (`volume.tsx`)
- **Purpose**: Volume/resonance status indicator
- **Props**: None (currently static)
- **Future**: Volume controls, settings integration

### 5. **Piano Component** (`piano.tsx`)
- **Purpose**: Complete piano keyboard
- **Props**: `activeNotes`, `onNoteOn`, `onNoteOff`
- **Features**: 88-key layout, white/black key positioning, footer legend

### 6. **Header Component** (`header.tsx`)
- **Purpose**: Top navigation bar
- **Props**: `selectedLayer`, `onLayerChange`, `layers`
- **Features**: Title, velocity selector, volume indicator

## 🎣 Custom Hook

### **usePiano Hook** (`usePiano.ts`)
- **Purpose**: Centralized piano logic and state management
- **Returns**: `activeNotes`, `selectedLayer`, `setSelectedLayer`, `layers`, `noteOn`, `noteOff`
- **Features**: 
  - MIDI note mapping
  - Velocity layer management
  - Keyboard event handling
  - Tauri API integration

## 📄 Pages

### **Home Page** (`home.tsx`)
- **Purpose**: Main application page (now simplified)
- **Structure**: Uses modular components
- **Code Size**: Reduced from 135 lines to 31 lines

### **Config Page** (`config.tsx`)
- **Purpose**: Future settings page
- **Status**: Empty (as requested)

## 🔄 Data Flow

```
usePiano Hook
    ↓ (state & functions)
Home Component
    ↓ (props)
Header Component ← Piano Component
    ↓ (props)
VelocitySelector ← WhiteKey/BlackKey Components
```

## ✅ Benefits of Modular Structure

1. **Reusability**: Components can be reused across pages
2. **Maintainability**: Smaller, focused components are easier to debug
3. **Testability**: Individual components can be unit tested
4. **Scalability**: Easy to add new features without affecting existing code
5. **Code Organization**: Clear separation of concerns
6. **Developer Experience**: Easier to understand and modify

## 🎯 Before vs After

**Before**: 135 lines in single `home.tsx` file
- All logic mixed together
- Hard to maintain and test
- Difficult to reuse components

**After**: 31 lines in `home.tsx` + modular components
- Clean separation of concerns
- Reusable components
- Easy to test and maintain
- Scalable architecture
